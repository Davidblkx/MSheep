use std::ffi::OsStr;
use std::path::PathBuf;

use crate::MusicData;
use crate::error::{MSheepError, Result};

static METADATA_KEYS: [&str; 7] = ["%artist%", "%album%", "%title%", "%track%", "%year%", "%albumartist%", "%disc%"];

pub fn build_path(template: &str, data: &MusicData, ext: Option<&OsStr>) -> Result<PathBuf> {
    let mut res = template.to_string();

    for key in METADATA_KEYS.iter() {
        if !res.contains(key) {
            continue;
        }

        let value = get_data_value(key, data);
        res = replace_template(res.as_str(), key, value)?;
    }

    if let Some(ext) = ext {
        match ext.to_str() {
            Some(e) => {
                res.push('.');
                res.push_str(e)
            },
            None => return Err(MSheepError::TaskError(format!("Invalid extension: {:?}", ext))),
        }
    }

    Ok(PathBuf::from(res))
}
        

fn get_data_value(key: &str, data: &MusicData) -> Option<String> {
    match key {
        "%artist%" => data.artist.clone(),
        "%album%" => data.album.clone(),
        "%title%" => data.title.clone(),
        "%track%" => data.track_number.map(|e| e.to_string()),
        "%year%" => data.year.map(|e| e.to_string()),
        "%albumartist%" => data.album_artist.clone(),
        "%disc%" => data.disc_number.map(|e| e.to_string()),
        _ => None,
    }
}

fn replace_template(path: &str, key: &str, value: Option<String>) -> Result<String> {
    let val = match value {
        Some(v) => clean_value(&v),
        None => return Err(MSheepError::TaskError(format!("No value for key: {}", key)))
    };

    Ok(path.replace(key, val.as_str()))
}

// replace invalid path characters by empty string
fn clean_value(value: &str) -> String {
    value.replace(&['\\', ':', '?', '*', '\'', '<', '>', '|', '/', '\0'][..], "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_value() {
        let value = "AC/DC";
        assert_eq!(clean_value(value), "ACDC");
        _ = PathBuf::from(clean_value(value));

        let value = "AC\\DC";
        assert_eq!(clean_value(value), "ACDC");
        _ = PathBuf::from(clean_value(value));

        let value = "AC\0DC";
        assert_eq!(clean_value(value), "ACDC");
        _ = PathBuf::from(clean_value(value));

        let value = "Back in Black: 2003";
        assert_eq!(clean_value(value), "Back in Black 2003");
        _ = PathBuf::from(clean_value(value));
    }
}