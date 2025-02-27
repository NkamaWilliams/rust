use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod area;
use area::*;
fn main() {
    let num = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..10 {
        let num = Arc::clone(&num);
        let handler = thread::spawn(move || {
            let mut num = num.lock().unwrap();
            *num += 1;
        });
        threads.push(handler);
    }

    for handler in threads {
        handler.join().unwrap();
    }

    println!("{num:?}");
    let circle = Circle::new(7 as f64);
    let poly = Polygon::new(circle);
    println!("{}", poly.get_area());
}

fn run() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {i} in spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number {i} in main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn run_multi_pass() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let v = vec![
            String::from("Hello"),
            String::from("World"),
            String::from(", "),
            String::from("Welcome"),
            String::from("To"),
            String::from("Rust!"),
        ];
        for val in v {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
