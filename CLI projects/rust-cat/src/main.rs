use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get the arguments passed to the program
    // skip(1) ignores the program name
    // collect() puts the remaining words into a list
    let args: Vec<String> = env::args().skip(1).collect();

    // Return if no arguments provided.
    if args.is_empty() {
        return;
    }

    // The filename user wants to see
    let filename_given_in_argument = &args[0];

    let file_result = File::open(filename_given_in_argument);

    let file = match file_result {
        Ok(file) => file,
        Err(error) => {
            println!("This error occurred: {}", error);
            return;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line)
            }

            Err(error) => {
                println!("{}", error)
            }
        }
    }
}
