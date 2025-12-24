pub struct Lexer<'a> {
    text: &'a str,
    pub current_character: char,
    current_position: usize,
}

impl<'a> Lexer<'a> {
    // Create an associated function. Due to convention, we name it "new"
    pub fn new(text: &'a str) -> Self {
        let mut lexer = Self {
            text,
            current_character: '\0',
            current_position: 0,
        };

        lexer.get_first_character();
        lexer
    }

    // Get the first character
    fn get_first_character(&mut self) {
        if let Some(ch) = self.text.chars().next() {
            self.current_character = ch;
        } else {
            self.current_character = '\0';
        }
    }

    // Process the next character
    pub fn next_character(&mut self) {
        self.current_position += 1;

        if self.current_position >= self.text.len() {
            self.current_character = '\0' // EOF
        } else {
            if let Some(char) = self.text.chars().nth(self.current_position) {
                self.current_character = char;
            } else {
                panic!("Failed to get char at next_character")
            }
        }
    }

    // Peek the next character without advancing the position
    // pub fn peek_character(&self) -> char {
    //     if self.current_position + 1 >= self.text.len() {
    //         '\0' // EOF
    //     } else {
    //         if let Some(char) = self.text.chars().nth(self.current_position + 1) {
    //             char
    //         } else {
    //             panic!("Failed to peek.")
    //         }
    //     }
    // }
}
