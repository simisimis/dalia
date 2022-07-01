use std::path::Path;

/// image_or_video determines whether a given path is an image or video file
pub(crate) fn image_or_video<P: AsRef<Path>>(filename: &P) -> bool {
    match infer::get_from_path(filename) {
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Image => true,
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Video => true,
        _ => false,
    }
}
