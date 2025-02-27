fn main() {
    let word = String::from("The fool that doesn't belong to this era!");
    analyzer(&word);
}

fn analyzer(s: &str) {
    let mut length = 0;
    let mut longest_word = "";
    let mut shortest_word = "";

    for (i, word) in s.split_whitespace().enumerate() {
        length += 1;
        if i == 0 {
            longest_word = word;
            shortest_word = word;
        }

        if longest_word.len() < word.len() {
            longest_word = word;
        }

        if shortest_word.len() > word.len() {
            shortest_word = word;
        }
    }

    println!("Number of words = {length}");
    println!("Longest word = {longest_word}");
    println!("Shortest word = {shortest_word}");
}
