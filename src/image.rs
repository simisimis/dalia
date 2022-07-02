use std::{io, path::Path};

/// image_or_video determines whether a given path is an image or video file
pub(crate) fn image_or_video<P: AsRef<Path>>(filename: &P) -> Result<bool, io::Error> {
    match infer::get_from_path(filename) {
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Image => Ok(true),
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Video => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(e),
        _ => Ok(false),
    }
}
