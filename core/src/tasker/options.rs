use std::path::PathBuf;
use crate::finder::MusicFileFinder;

/// Options for the tasker
pub struct TaskOptions {
    /// The root directory where the music files are located.
    pub root: PathBuf,
    /// If true, will search for music files in subdirectories.
    pub recursive: bool,
    /// If true, the tasker will not perform any changes to the music files.
    pub dry_run: bool,
}

impl TaskOptions {
    pub fn build_finder(&self) -> MusicFileFinder {
        MusicFileFinder::new(self.root.clone())
            .with_recursive(self.recursive)
    }
}