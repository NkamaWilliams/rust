use rand::Rng;
use std::cmp::Ordering;
use std::io;

//My variant of the guessing game which gives a set number of tries
fn main() {
    println!("I'm thinking of a number between 1 and 100. Guess the number!");

    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    let mut tries = 8;
    // println!("The secret number is {secret}");

    while tries > 0 {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = guess.trim().parse().expect("Please input a number!");

        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Too small!");
                tries -= 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                tries -= 1;
            }
            Ordering::Equal => {
                println!("You guessed the secret number correctly!");
                break;
            }
        }
    }

    if tries <= 0 {
        print!("You lose! The secret number was {secret}");
    }
}
