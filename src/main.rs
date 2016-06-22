use std::io::{self, Write};

fn main() {
    print!("stutter version 0.0.1\n");
    print!("Press Ctrl+c to exit\n");
    print!("stutter>");

    loop {
        io::stdout().flush().unwrap();
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        println!("No, you're a {}", input);
        print!("stutter>");
    }
}
