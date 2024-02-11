use std::path::PathBuf;

use msheep_core::MusicFile;

fn main() {
    let path = PathBuf::from("D:\\Music\\acdc_music.mp3");
    let mut music_file = MusicFile::new(path);
    music_file.load_data().unwrap();
    if let Some(data) = music_file.data {
        println!("{}", data);
    }
}
