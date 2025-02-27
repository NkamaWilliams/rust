use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath)?;

    if config.ignore_case {
        for line in search_insensitive(&config.query, &contents) {
            println!("{line}")
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}")
        }
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            filepath,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
