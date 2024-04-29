use std::io;
use rand::Rng;
use colored::*;
fn main() {

    let secret_num = rand::thread_rng().gen_range(1..=10);

    println!("Welcome to the guessing game!");

    'game_loop: loop {
        println!("Please enter a number between 0 and 10: ");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read number");

        let guessed_num = match user_input.trim().parse::<u32>(){
            Ok(num) => num,
            Err(_) => return,
        };

        println!("You guessed: {guessed_num}");

        // println!("Your guess is correct {}", guessed_num == secret_num);
        if guessed_num == secret_num {
            println!("Your guess is correct");
            println!("{}","Your guess is correct".green());
            break 'game_loop;
        } else {
            if guessed_num < secret_num {
                println!("{}","Your guess is too low!".red());
            }else{
                println!("{}","Your guess is too high".red());
            }
            
        }
    }
}