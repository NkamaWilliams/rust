fn main() {
    let word = String::from("Hello world!");
    println!("{}", &word[0..first_word(&word)]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{i}");
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
