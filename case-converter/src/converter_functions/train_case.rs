use crate::lexer;

pub fn convert_to_train_case(text: &str) -> String {
    let mut result = String::from("");
    let mut lexer = lexer::Lexer::new(text);

    // Empty text
    if text.is_empty() {
        panic!("Text is empty");
    };

    while lexer.current_character != '\0' {
        // First letter must be uppercase.
        if lexer.current_position == 0 && lexer.current_character.is_lowercase() {
            result.extend(lexer.current_character.to_uppercase());
            lexer.next_character();

            continue;
        }

        // Character after each whitespace must be uppercase.
        if lexer.current_character == ' ' && lexer.peek_character() != '\0' {
            // Add '-' to result for whitespace character
            result.push('-');
            lexer.next_character();

            result.extend(lexer.current_character.to_uppercase());
            lexer.next_character();

            continue;
        }

        // Anything else just gets appended.
        result.push(lexer.current_character);
        lexer.next_character();
    }

    result
}
