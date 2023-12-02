use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Welcome to guess the number game");
    println!("I will pick a number between 1 and 100, and you try to guess it");
    println!("Ready? Let's go!");

    let answer = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("");
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&answer){
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Greater => println!("Your guess is too big"),
            Ordering::Equal => {
                println!("You are correct. The answer is {answer}");
                break;
            }
        }
    }
}