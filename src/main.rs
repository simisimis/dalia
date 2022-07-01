mod directory;
mod image;

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

    println!("path provided: {}!", args.path);

    for i in directory::get_file_iter(args.path, args.skip_type_checking).unwrap() {
        println!("{}", i.to_string_lossy());
    }
}
