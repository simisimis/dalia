use std::{io, path::Path};

/// image_or_video determines whether a given path is an image or video file
pub fn image_or_video<P: AsRef<Path>>(filename: &P) -> Result<bool, io::Error> {
    match infer::get_from_path(filename) {
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Image => Ok(true),
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Video => Ok(true),
        Ok(_) => Ok(false),
        Err(e) => Err(e),
    }
}
