use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::image;

pub(crate) fn get_file_iter(root: String) -> Result<Vec<PathBuf>, io::Error> {

    let mut v: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {

        let f_path = entry.path();
        // Check the file type
        //ALTERNATIVE ==> f_path.is_file() && info.is_image(&fs::read(f_path).unwrap())
        if image::image_or_video(&f_path) {
            v.push(PathBuf::from(f_path));
        }
    }
    Ok(v)
}