use std::path::PathBuf;
use audiotags::Tag;

use crate::MusicData;
use crate::error::Result;

pub struct MusicFile {
    pub path: PathBuf,
    pub data: Option<MusicData>,
}

impl MusicFile {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            data: None,
        }
    }

    pub fn load_data(&mut self) -> Result<()> {
        self.data = Some(self.read_data()?);
        Ok(())
    }

    pub fn read_data(&self) -> Result<MusicData> {
        log::trace!("Reading data from file: {:?}", self.path);
        let tag = Tag::new().read_from_path(&self.path)?;
        Ok(MusicData::from_audiotag(tag))
    }
    
}

impl From<PathBuf> for MusicFile {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<String> for MusicFile {
    fn from(path: String) -> Self {
        Self::new(path.into())
    }
}

impl From<&str> for MusicFile {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}