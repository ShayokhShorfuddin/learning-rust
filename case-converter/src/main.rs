// The goal of this program is to convert a given string into different types of cases. For example, converting to uppercase, lowercase, camelCase, snake_case, etc.
use std::io;

mod converter_functions;
mod lexer;

fn main() {
    println!("Enter your text: ");

    // Create a mutable string to hold the user input
    let mut text = String::new();

    // Create a mutable string to hold the user's desired case
    let mut desired_case = String::new();

    // Read input from terminal.
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to readline");

    println!("Enter the desired case: ");

    io::stdin()
        .read_line(&mut desired_case)
        .expect("Failed to readline");

    // Trimming the trailing newline (ENTER) character. This gets added to the input string because we press ENTER after typing the input.
    let text = text.trim();

    // UPPERCASE,
    // lowercase,
    // camelCase,
    // UpperCamelCase,
    // snake_case,
    // KebabCase,
    // ScreamingSnakeCase,
    // TrainCase,

    match desired_case.trim() {
        "UPPERCASE" => println!("{}", text.to_uppercase()),
        "lowercase" => println!("{}", text.to_lowercase()),

        "camelCase" => println!(
            "{}",
            converter_functions::camelcase::convert_to_camel_case(&text)
        ),
        "UpperCamelCase" => println!(
            "{}",
            converter_functions::uppercamelcase::convert_to_upper_camel_case(&text)
        ),
        "kebab-case" => println!(
            "{}",
            converter_functions::kebab_case::convert_to_kebab_case(&text)
        ),
        _ => println!("Unknown case. Sorry."),
    }
}
