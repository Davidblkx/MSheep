use std::fmt::Display;

use audiotags::AudioTag;

pub struct MusicData {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub artists: Vec<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_artists: Vec<String>,
    pub composer: Option<String>,
    pub track_number: Option<u32>,
    pub total_tracks: Option<u32>,
    pub disc_number: Option<u32>,
    pub total_discs: Option<u32>,
    pub year: Option<u32>,
    pub genre: Option<String>,
}

impl MusicData {
    pub fn from_audiotag(tag: Box<dyn AudioTag + Send + Sync>) -> Self {
        let artists: Vec<String> = tag.artists()
            .map(|e| e.into_iter().map(|v| v.to_string()).collect())
            .unwrap_or(vec![]);
        let album_artists: Vec<String> = tag.album_artists()
            .map(|e| e.into_iter().map(|v| v.to_string()).collect())
            .unwrap_or(vec![]);

        Self {
            title: tag.title().map(|e| e.to_string()),
            artist: artists.first().map(|e| e.to_string()),
            album_artist: album_artists.first().or(artists.first()).map(|e| e.to_string()),
            artists,
            album: tag.album_title().map(|e| e.to_string()),
            album_artists,
            track_number: tag.track_number().map(|e| e as u32),
            total_tracks: tag.total_tracks().map(|e| e as u32),
            disc_number: tag.disc_number().map(|e| e as u32),
            total_discs: tag.total_discs().map(|e| e as u32),
            year: read_year(&*tag),
            composer: tag.composer().map(|e| e.to_string()),
            genre: tag.genre().map(|e| e.to_string()),
        }
    }
}

impl Default for MusicData {
    fn default() -> Self {
        Self {
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            track_number: None,
            total_tracks: None,
            disc_number: None,
            total_discs: None,
            year: None,
            album_artists: vec![],
            artists: vec![],
            composer: None,
            genre: None,
        }
    }
}

impl Display for MusicData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} - {} - {} - {} - {} - {} - {} - {}",
            self.title.as_deref().unwrap_or(""),
            self.artist.as_deref().unwrap_or(""),
            self.album.as_deref().unwrap_or(""),
            self.album_artist.as_deref().unwrap_or(""),
            self.track_number.map(|e| e.to_string()).unwrap_or("".to_string()),
            self.total_tracks.map(|e| e.to_string()).unwrap_or("".to_string()),
            self.disc_number.map(|e| e.to_string()).unwrap_or("".to_string()),
            self.total_discs.map(|e| e.to_string()).unwrap_or("".to_string()),
            self.year.map(|e| e.to_string()).unwrap_or("".to_string())
        )
    }
}

fn read_year(tag: &dyn AudioTag) -> Option<u32> {
    match tag.year() {
        Some(year) => Some(year as u32),
        None => match tag.date() {
            Some(date) => Some(date.year as u32),
            None => None,
        },
    }
}
