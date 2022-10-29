use std::{fs, io, path::Path};

/// Error enumerates all errors returned by mimetype module.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Forward underlying `std::io` errors.
    #[error(transparent)]
    IOError(#[from] io::Error),
}
/// image_or_video determines whether a given path is an image or video file
pub fn image_or_video<P: AsRef<Path>>(path: &P) -> Result<bool, Error> {
    if fs::metadata(path)?.is_dir() {
        Ok(false)
    } else {
        match infer::get_from_path(path) {
            Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Image => Ok(true),
            Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Video => Ok(true),
            Ok(_) => Ok(false),
            Err(e) => Err(Error::IOError(e)),
        }
    }
}
