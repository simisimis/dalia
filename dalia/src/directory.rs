use crate::mimetype;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_file_iter(root: String, skip_type_checking: bool) -> Result<Vec<PathBuf>, io::Error> {
    let mut v: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_path = entry.path();
        // Check the file type
        //ALTERNATIVE ==> f_path.is_file() && info.is_image(&fs::read(f_path).unwrap())
        if !skip_type_checking {
            match mimetype::image_or_video(&f_path) {
                Ok(true) => v.push(PathBuf::from(f_path)),
                Ok(false) => {}
                Err(err) => log::error!(
                    "Failed to determine if path: {} is image or video. Error: {}",
                    &f_path.display(),
                    err
                ),
            }
        } else {
            v.push(PathBuf::from(f_path));
        }
    }
    Ok(v)
}
