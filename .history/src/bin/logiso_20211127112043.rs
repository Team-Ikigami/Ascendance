use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    id: i32,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
struct entitymovement {
    x: i32,
    y: i32,
    z: i32,
    entitytype: i32,
}

fn main() {

    match id {
        1 = {
            println!("{}", id);
        }
        2 = {
            println!("{}", id);
        }
        3 = {
            println!("{}", id);
        }
        4 = {
            println!("{}", id);
        }
        5 = {
            println!("{}", id);
        }
        _ = {
            println!("{}", id);
        }
    }
}