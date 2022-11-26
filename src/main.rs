/**
 * guessing_game
 * By Edward Naidoo
 * Credits: The Rust Book, Lets Get Rusty
 * Nov 25, 2022
 */

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    loop {
        let mut guess = String::new();
        
        // Generated Variable Value
        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("The secret number is: {}", secret_number);
    
        println!("Guess the number!");
        println!("Please input your guess.");
    
        // Collects user input and puts it in "guess"
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}", "Correct!".green().bold());
                break;
            },
            Ordering::Less => println!("{}", "Too Small".yellow())
        }
    }
}
