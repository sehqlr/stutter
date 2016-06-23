extern crate readline;

fn echo_and_add(line: String) {
    println!("No, you're a {}", line);
    match readline::add_history(&line) {
        Ok(_) => return (),
        Err(e) => panic!(e)
    }
    
}

fn main() {
    print!("stutter version 0.0.1\n");
    print!("Press Ctrl+c to exit\n");

    loop {
        match readline::readline("stutter> ") {
            Ok(line) => echo_and_add(line),
            Err(_) => break
        }
    }
}
