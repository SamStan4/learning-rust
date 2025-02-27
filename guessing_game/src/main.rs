use std::io::{self, Write}; // this brings the io library into scope

fn main() {
    println!("Guessing Game");

    print!("Enter a number> "); // In rust, when there is no new line character, you must manually flush the buffer
    io::stdout()
        .flush()
        .unwrap();

    // mut makes it so the guess variable is a mutable variable
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("System error: failed to read line");

    println!("Here is what you guessed: {}", guess);
}
