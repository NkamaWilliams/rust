pub mod maps;
pub use maps::*;
fn main() {
    println!("Hello, world!");
    let start = std::time::Instant::now();
    heavy_func();
    let duration = std::time::Instant::now().duration_since(start);
    println!("{}", duration.as_micros());
}

fn heavy_func() {
    let num = 9999;
    let num2 = 99999;
    let num3: i64 = num * num2;
    println!("{}", num3);
}
