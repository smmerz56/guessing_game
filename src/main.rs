use colored::*;
use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..=10);

    println!("Welcome to a Guessing Game!\n");

    loop {
        println!("Please enter a number between 0 and 10:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");

        let guessed_number = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => return,
        };

        if guessed_number == secret_number {
            println!("{}", "Your guess is correct".blue());
            break;
        } else {
            if secret_number < guessed_number {
                println!("{}", "Your number is too big! Guess again!".red());
            } else {
                println!("{}", "Your number is too small! Guess again!".red());
            }
        }
    }
}