mod directory;
mod image;
mod metadata;
use clap::Parser;

/// An utility to do magic with your photos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A path to search photos in
    #[clap(short, long, value_parser)]
    path: String,

    /// Skip recognising file types
    #[clap(short, long, value_parser)]
    skip_type_checking: bool,
}

fn main() {
    let args = Args::parse();

    println!("Provided path: {}", args.path);

    for p in directory::get_file_iter(args.path, args.skip_type_checking).unwrap() {
        match metadata::read_metadata(p.as_path()) {
            Ok(metadata) => {
                eprintln!("File: {}", p.to_string_lossy());
                eprintln!("{:?}", metadata);
                eprintln!("Date time created: {:?}", metadata.date_time_created);
                eprintln!("Date time taken: {:?}", metadata.date_time_taken);
            }
            Err(err) => eprintln!(
                "Could not read metadata for {}: {}",
                p.to_string_lossy(),
                err
            ),
        }
        eprintln!("---");
    }
}
