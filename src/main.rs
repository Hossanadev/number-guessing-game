use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
       loop {
        let mut num_guess: String = String::new();
        let secret_number = rand::thread_rng().gen_range(1..=10);
        println!("Enter your guess:");
        io::stdin()
                .read_line(& mut  num_guess)
                .expect("Failed to read line");
                let num_guess: u32 = match num_guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue
                };
        println!("Your guess is {num_guess}. The secret number is {secret_number}.");
        match num_guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Your guess is less than the secret number, TRY AGAIN!!".red()),
            Ordering::Greater => println!("{}", "Your guess is greater than the secret number, TRY AGAIN!!".yellow()),
            Ordering::Equal => {
                println!("{}","YOU WIN!!!".green());
                break;
            }
        }
    }
}

// To understand this code properly, you need to read "Chapter 2" of "The Rust Programming Language".
// Hossanadev :)