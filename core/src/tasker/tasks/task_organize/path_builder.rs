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

    // TODO: probably we want review this, it's not clear if we want to lowercase the whole path
    //       but it helps to avoid issues with case sensitive file systems
    Ok(PathBuf::from(res.to_lowercase()))
}

fn get_data_value(key: &str, data: &MusicData) -> Option<String> {
    match key {
        "%artist%" => data.artist.clone(),
        "%album%" => data.album.clone(),
        "%title%" => data.title.clone(),
        "%track%" => data.track_number.map(|e| format!("{:0>2}", e)),
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
    let mut val = value
        .replace('"', "'")
        .replace(&['\\', ':', '?', '*', '<', '>', '|', '/', '\0'][..], "");

    while val.ends_with('.') {
        let len = val.len();
        val.truncate(len - 1);   
    }

    // reduce if bigger than 101
    match val.len() {
        0..=101 => val,
        _ => reduce_string(&val, 100),
    }
}

// Pick first letter of every word in the string until max length
fn reduce_string(value: &str, max: usize) -> String {
    let mut res = String::new();
    for word in value.split_whitespace() {
        if res.len() >= max {
            break;
        }
        res.push_str(&word.chars().next().unwrap().to_string());
    }

    res
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

        let value = "The Idler Wheel Is Wiser Than the Driver of the Screw and Whipping Cords Will Serve You More Than Ropes Will Ever Do";
        assert_eq!(clean_value(value), "TIWIWTtDotSaWCWSYMTRWED");
        _ = PathBuf::from(clean_value(value));

        let value = "AC/DC.";
        assert_eq!(clean_value(value), "ACDC");
        _ = PathBuf::from(clean_value(value));
    }
}