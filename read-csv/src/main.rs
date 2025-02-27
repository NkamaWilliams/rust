use std::{error::Error, result};
use csv;
use std::env;
use std::process;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records(){
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: command pathname");
        process::exit(0);
    }
    if let Err(e) = read_from_file(&args[1].as_str()){
        eprintln!("{}", e);
    }
}
