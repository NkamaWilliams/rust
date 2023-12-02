use std::io;
fn main() {
    println!("Hello, I am IZO! What's your name?");
    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Failed to get name");

    println!("Welcome to Rust {name}");
}
