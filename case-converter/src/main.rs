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

    // UpperCase,
    // LowerCase,
    // CamelCase,
    // UpperCamelCase,
    // SnakeCase,
    // PascalCase,
    // KebabCase,
    // ScreamingSnakeCase,
    // TrainCase,

    match desired_case.trim().to_lowercase().as_str() {
        "uppercase" => println!("{}", text.to_uppercase()),
        "lowercase" => println!("{}", text.to_lowercase()),

        "kebabcase" => println!(
            "{}",
            converter_functions::kebab_case::convert_to_kebab_case(&text)
        ),
        // "lowercase" => println!("{}", text.to_lowercase()),
        // "lowercase" => println!("{}", text.to_lowercase()),
        // "lowercase" => println!("{}", text.to_lowercase()),
        // "lowercase" => println!("{}", text.to_lowercase()),
        // "lowercase" => println!("{}", text.to_lowercase()),
        _ => println!("Unknown case. Sorry."),
    }
}

// fn to_camelcase(text: String) {
//     let mut result = "";

//     for char in text.chars() {}
// }

// fn main() {
//     let mut lexer = lexer::Lexer::new("Hello".chars().collect());

//     while lexer.current_character != '\0' {
//         println!("Current character: {}", lexer.current_character);
//         lexer.next_character();
//     }
// }
