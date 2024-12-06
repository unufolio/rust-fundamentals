use std::env;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: file-reader <filename>");
        return;
    }

    let filename = &args[1];
    read_file(filename);
}

fn read_file(filename: &str) {
    let file = File::open(filename);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => {
                        println!("Error reading line: {}", error)
                    }
                }
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File not found: {}", error)
            }
            _ => {
                println!("Error opening file: {}", error)
            }
        },
    };
}
