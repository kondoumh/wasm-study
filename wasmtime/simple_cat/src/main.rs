extern crate structopt;

use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

#[derive(StructOpt)]
#[structopt(name = "simple_cat", about = "Simple cat program")]
pub struct Options {
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

fn main() {
    let options = Options::from_args();
    let contents = fs::read_to_string(options.filename)
        .expect("Something went wrong reading the file");

    // Print out the resulting HTML to standard out
    println!("{}", contents);
}
