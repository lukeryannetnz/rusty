extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Error reading line of text");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("higher..."),
            Ordering::Greater => println!("lower..."),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
}
