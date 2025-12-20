// This is a single-line comment
// Followed by another single-line comment

fn main() {
    test_welcome();
}

// The 'static lifetime tells Rust this string lives for the entire duration of the program,
// which is true for all hard-coded text.

fn greeting() -> &'static str {
    return "I am ready to learn Rust!";
}

fn test_welcome() {
    assert_eq!(greeting(), "I am ready to learn Rust!")
}
