mod directory;
mod metadata;
mod mimetype;
use clap::Parser;

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
}

fn main() {
    let args = Args::parse();

    println!("Provided path: {}", args.path);

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
