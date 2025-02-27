use std::{fmt::format, result};

fn main() {
    println!("Hello, world!");
    println!("{:#?}", gen_parenthese(3));
}

fn gen_parenthese(n: u64) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    fn backtrack(s: String, left: u64, right: u64, result: &mut Vec<String>, n: u64) {
        println!("{:#?}", s);
        if s.len() == 2 * n as usize {
            result.push(s);
            return;
        }
        if left < n {
            backtrack(format!("{}(", s), left + 1, right, result, n);
        }
        if right < left {
            backtrack(format!("{})", s), left, right + 1, result, n);
        }
    }
    backtrack(String::from(""), 0, 0, &mut result, n);
    result
}
