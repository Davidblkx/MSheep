use std::path::PathBuf;

use crate::error::{MSheepError, Result};

use super::config::OrganizeConfig;

pub fn organize_file(cfg: &OrganizeConfig, from: &PathBuf, to: &PathBuf) -> Result<()> {
    create_folders(to)?;
    if cfg.r#move {
        log::debug!("Moving file: {} -> {}", from.display(), to.display());
        match std::fs::rename(from, to) {
            Ok(_) => (),
            Err(e) => {
                if !cfg.copy_on_fail {
                    return Err(MSheepError::IOError(e));
                }
                log::error!("Failed to move file: {}, trying to copy instead", e);
                std::fs::copy(from, to)?;
                std::fs::remove_file(from)?;
            }
        }
    } else {
        log::debug!("Copying file: {} -> {}", from.display(), to.display());
        std::fs::copy(from, to)?;
    }

    Ok(())
}


fn create_folders(path: &PathBuf) -> Result<()> {
    match path.parent() {
        Some(parent) => std::fs::create_dir_all(parent)?,
        None => return Err(MSheepError::TaskError("Invalid path".to_string())),
    }

    Ok(())
}