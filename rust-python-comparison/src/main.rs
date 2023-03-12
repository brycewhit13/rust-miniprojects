// Crates
use rust_python_comparison::{count_to_billion, load_airport_data};
use std::time::Instant;

// Main Function
fn main() {
    let start = Instant::now();
    count_to_billion();
    let duration = start.elapsed().as_millis();
    println!("Rust: count_to_billion took {} milliseconds", duration);
    
    let start = Instant::now();
    let _df = load_airport_data();
    let duration = start.elapsed().as_millis();
    println!("Rust: load_airport_data took {} milliseconds", duration);
}   
