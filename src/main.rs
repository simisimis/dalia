use clap::Parser;
use dalia::directory;
use dalia::metadata;

/// An utility to do magic with your photos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A path to search photos in
    #[clap(short, long, value_parser, default_value = ".")]
    path: String,

    /// Skip recognising file types
    #[clap(short, long)]
    skip_type_checking: bool,

    /// Set required logging level.
    #[clap(long, default_value = log::LevelFilter::Info.as_str())]
    log_level: log::LevelFilter,
}

fn main() {
    let args = Args::parse();
    pretty_env_logger::formatted_builder()
        .filter(None, args.log_level)
        .init();

    log::info!("Provided path: {}", args.path);

    for p in directory::get_file_iter(args.path, args.skip_type_checking).unwrap() {
        println!("File: {}", p.to_string_lossy());
        match metadata::read_metadata(p.as_path()) {
            Ok(metadata) => {
                println!("{}", metadata);
            }
            Err(err) => eprintln!("Could not read metadata: {}", err),
        }
        println!("---");
    }
}
