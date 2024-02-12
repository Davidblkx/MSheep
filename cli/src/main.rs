use std::path::PathBuf;

use msheep_core::finder::MusicFileFinder;

fn main() {
    let path = PathBuf::from("E:\\music");
    let finder = MusicFileFinder::new(path).with_recursive(true);
    let list = finder.list().unwrap();
    for file in list {
        let mut file = file.unwrap();
        file.load_data().unwrap();
        println!("{:?}", file.path);
    }
}
