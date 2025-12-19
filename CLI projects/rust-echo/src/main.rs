use std::env;

fn main() {
    // skip(1) ignores the program name
    // collect() puts the remaining words into a list
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        // Handle the case where the user provided no text
        return;
    }

    println!("{}", args.join(" "));
}
