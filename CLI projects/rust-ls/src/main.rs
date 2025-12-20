use std::fs;

fn main() {
    let entries_result = fs::read_dir(".");

    let entires = match entries_result {
        Ok(entires) => entires,

        Err(error) => {
            println!("This error occurred: {}", error);
            return;
        }
    };

    // TODO: Implement it later. This one is a bit tricky.
    for entry in entires {
        match entry {
            Ok(entry) => {
                if let Some(name) = entry.path().file_name() {
                    // println!("{}", name);
                }
            }

            Err(error) => {
                println!("This error occurred: {}", error)
            }
        }
    }
}
