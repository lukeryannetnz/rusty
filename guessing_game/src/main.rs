extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);

    println!("Input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Error reading line of text");

    let guess: u32 = guess.trim().parse()
        .expect("Error! Please type a number.");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("higher..."),
        Ordering::Greater => println!("lower..."),
        Ordering::Equal => println!("YOU WIN!")
    }
}
