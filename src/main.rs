mod image;
mod directory;

use clap::Parser;

/// An utility to do magic with your photos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A path to search photos in
    #[clap(short, long, value_parser)]
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("path provided: {}!", args.path);

    for i in directory::get_file_iter(args.path).unwrap() {
        println!("{}", i.to_string_lossy());
    }

}
