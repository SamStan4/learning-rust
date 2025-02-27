use std::io::{self, Write}; // this brings the io library into scope
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number: u32 = rand::rng().random_range(1..=100);
    
    println!("Guessing Game");

    loop {
        // In rust, when there is no new line character, you must manually flush the buffer
        print!("Enter a number> ");
        io::stdout()
            .flush()
            .unwrap();

        // mut makes it so the guess variable is a mutable variable
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .unwrap();

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Enter a valid value");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small".red());
            },
            Ordering::Greater => {
                println!("{}", "Too big".red());
            },
            Ordering::Equal => {
                println!("{}", "Just right".green());
                break;
            }
        }
    }
}
