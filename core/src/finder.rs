use std::path::PathBuf;

use crate::{error::Result, MusicFile};

#[derive(Debug, Clone)]
pub struct MusicFileFinder {
    pub root: PathBuf,
    pub extensions: Vec<String>,
    pub recursive: bool,
}

impl MusicFileFinder {
    pub fn new(root: PathBuf) -> Self {
        MusicFileFinder {
            root,
            ..Default::default()
        }
    }

    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    pub fn list(&self) -> Result<MusicDirResult> {
        let read_dir = self.root.read_dir()?;
        Ok(MusicDirResult::new(read_dir, self.extensions.clone(), self.recursive))
    }
}

impl Default for MusicFileFinder {
    fn default() -> Self {
        MusicFileFinder {
            root: PathBuf::from("."),
            extensions: build_default_extensions(),
            recursive: false,
        }
    }
}

fn build_default_extensions() -> Vec<String> {
    vec![
        "mp3",
        "flac",
        "mp4",
        "m4a",
        "m4b",
        "m4p",
        "m4r",
    ].into_iter()
    .map(|s| s.to_string())
    .collect()
}

pub struct MusicDirResult {
    read_dir: std::fs::ReadDir,
    parent: Vec<std::fs::ReadDir>,
    extensions: Vec<String>,
    recurse: bool,
}

impl MusicDirResult {
    pub fn new(read_dir: std::fs::ReadDir, ext: Vec<String>, recurse: bool) -> Self {
        MusicDirResult { read_dir, extensions: ext, recurse, parent: vec![] }
    }
}

impl Iterator for MusicDirResult {
    type Item = Result<MusicFile>;

    fn next(&mut self) -> Option<Result<MusicFile>> {
        loop {
            match self.read_dir.next() {
                None => {
                    match self.parent.pop() {
                        None => return None,
                        Some(parent) => {
                            self.read_dir = parent;
                            continue;
                        }
                    }
                },
                Some(Err(e)) => return Some(Err(e.into())),
                Some(Ok(entry)) => {
                    match entry.file_type() {
                        Err(e) => return Some(Err(e.into())),
                        Ok(file_type) => {
                            if file_type.is_dir() && self.recurse {
                                match entry.path().read_dir() {
                                    Err(e) => return Some(Err(e.into())),
                                    Ok(read_dir) => {
                                        let old_read_dir = std::mem::replace(&mut self.read_dir, read_dir);
                                        self.parent.push(old_read_dir);
                                        continue;
                                    }
                                }
                            } else if file_type.is_file() {
                                let path = entry.path();
                                if let Some(ext) = path.extension() {
                                    if self.extensions.contains(&ext.to_string_lossy().to_string()) {
                                        return Some(Ok(MusicFile::new(path)));
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}