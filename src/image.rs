use std::path::Path;

pub fn image_or_video<P: AsRef<Path>>(filename: &P) -> bool {
    match infer::get_from_path(filename) {
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Image => {
            true
        }
        Ok(Some(info)) if info.matcher_type() == infer::MatcherType::Video => {
            true
        }
        Err(e) => {
            eprintln!("Looks like something went wrong 😔");
            eprintln!("{}", e);
            false
        }
        _ => { false }
    }
}
