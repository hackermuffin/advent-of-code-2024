use clap::Parser;
use core::panic;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

mod days;
mod shared;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    day: i32,

    #[arg(short, long, value_name = "FILE")]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let input = match args.input {
        None => {
            // Read input from stdin
            let mut buffer = String::new();
            if let Err(why) = io::stdin().read_to_string(&mut buffer) {
                panic!("Couldn't read from stdin: {}", why)
            }
            buffer
        }
        Some(path) => {
            // Read input from file
            let path = Path::new(&path);
            let display = path.display();
            let mut file = match File::open(path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };
            let mut s = String::new();
            if let Err(why) = file.read_to_string(&mut s) {
                panic!("couldn't read {}: {}", display, why)
            };
            s
        }
    };

    days::run_day(args.day, input);
}
