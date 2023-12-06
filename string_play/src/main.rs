use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Hello, user. Give me a string to play with or I'll gut you!");
    let mut name = String::new();
    let mut surname = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read string");

    io::stdin()
        .read_line(&mut surname)
        .expect("Failed to read surname");

    name.push_str(" ");
    name.push_str(&mut surname);

    let name2 = name;
    println!("You inputted {name2}");

    let me = String::from("Joker");
    match me.cmp(&name2){
        Ordering::Less => println!("Try harder next time. What I wanted was {me}"),
        Ordering::Equal => println!("You guessed what I had in mind"),
        Ordering::Greater => println!("Try again later loser, the correct answer is {me}")
    }
}   
