use crate::lexer;

pub fn convert_to_snake_case(text: &str) -> String {
    let mut result = String::from("");
    let mut lexer = lexer::Lexer::new(text);

    // Empty text
    if text.is_empty() {
        panic!("Text is empty");
    };

    while lexer.current_character != '\0' {
        // Converting all characters to lowercase
        if lexer.current_character.is_uppercase() {
            result.extend(lexer.current_character.to_lowercase());
            lexer.next_character();

            continue;
        }

        // Turn whitespaces into '_'
        if lexer.current_character == ' ' {
            result.push('_');
            lexer.next_character();

            continue;
        }

        // Anything else just gets appended.
        result.push(lexer.current_character);
        lexer.next_character();
    }

    result
}
