// Write a program to play the higher or lower guessing game:
// 1. The program generates a random number between 1 and 100.
// 2. The user tries to guess the secret number.
// 3. The program tells the user if their guess was too high, too low, or correct.
// 4. Repeat steps 2 and 3 until the user guesses correctly.

use rand::Rng;
use std::io;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Guess the secret number! (an integer between 1 and 100):");

        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        let user_guess: i32 = buffer.trim().parse().unwrap();

        if user_guess == secret_number {
            println!("You guessed the number! Congrats!");
            break;
        } else if user_guess > secret_number {
            println!("The secret number is lower than that.");
        } else {
            println!("The secret number is higher than that.");
        }
    }
}
