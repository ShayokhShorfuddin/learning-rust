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

    // The word to search
    let word = &args[0];

    // The filename to look into
    let filename = &args[1];

    // State to determine if any match was found
    let mut at_least_one_match_found = false;

    let file_result = File::open(filename);

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
                // Check if word is in the line
                if line.contains(word) {
                    println!("{}", line);
                    at_least_one_match_found = true;
                }
            }

            Err(_) => {
                panic!("Failed to read line.")
            }
        }
    }

    if !at_least_one_match_found {
        println!("No match found.");
    }
}
