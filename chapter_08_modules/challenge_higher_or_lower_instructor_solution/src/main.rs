// Barron Stone's solution originally, but heavily refactored by me:
// - all logic separated into dedicated functions
// - give the user a max of 5 retries to guess the secret number
// - give the user the option to play again using the Rust `match`

use rand::prelude::*;
use std::io;

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("\nI'm thinking of a number between 1 and 100...");
        println!("Guess the number:");
        guess_number(secret_number);

        let confirmation = continue_game();
        if !confirmation {
            break;
        }
    }
}

fn inc_counter(counter: &mut u32) {
    *counter += 1
}

fn guess_number(secret_number: u32) {
    let mut counter: u32 = 0;

    loop {
        if counter == 5 {
            println!("\nYou reached the max number of retries. You lost!");
            break;
        }

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line.");
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
        inc_counter(&mut counter); // 5 guesses total
    }
}

fn continue_game() -> bool {
    let mut confirmation = String::new();
    println!("\nPress 0 to quit or 1 to continue:");
    io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read the input");
    let confirmation: u32 = confirmation
        .trim()
        .parse()
        .expect("Failed to parse the input");

    let confirmation = match confirmation {
        0 => false, // If the integer is 0, it's false
        _ => true,  // For any other integer, it's true
    };
    confirmation
}
