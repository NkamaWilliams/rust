use std::io;

//This program gives a twist to the classic FizzBuzz challenge
//Given a number, it counts down to 0, printing Fizz, Buzz or FizzBuzz as appropriate
fn main() {
    println!("Hi, I am FizzBuzz Countdown!");
    println!("Input a number for me to start my countdown from:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read number");

    let mut num: u32 = num
        .trim()
        .parse()
        .expect("Please input a valid number greater than zero");

    while num > 0 {
        println!("{num}");
        fizzbuzz(num);
        num -= 1;
    }

    println!("Lift Off!");
}

fn fizzbuzz(num: u32) {
    if num % 3 == 0 && num % 5 == 0 {
        println!("FizzBuzz");
    } else if num % 3 == 0 {
        println!("Fizz");
    } else if num % 5 == 0 {
        println!("Buzz");
    }
}
