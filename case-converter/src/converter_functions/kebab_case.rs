use crate::lexer;

pub fn convert_to_kebab_case(text: &str) -> String {
    let mut result = String::from("");
    let mut lexer = lexer::Lexer::new(text);

    // Empty text
    if char_vector.is_empty() {
        panic!("Text is empty");
    };

    while lexer.current_character != '\0' {
        println!("Current character: {}", lexer.current_character);
        lexer.next_character();
    }

    // for char in char_vector {

    // }

    String::from("Hahah!")
}

// TODO: Learn Rust lifetime, UTF-8 encoding
