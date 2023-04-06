use clap::Parser;
use fibonacci::{fibonacci_iterative, fibonacci_recursive};

#[derive(Parser, Debug)]
#[clap(
    version = "4.2.1",
    author = "Bryce Whitney",
    about = "A command-line tool that gets the nth fibonacci number iteratively and recursively.",
    after_help = "Example: cargo run -- -n 10"
)]

struct Args {
    // Argument for the nth fibonacci number to get.
    #[clap(short, long, default_value = "10")]
    num: u32,
}

fn main() {
    let n = Args::parse().num;
    println!("fibonacci_recursive({}) = {}", n, fibonacci_recursive(n));
    println!("fibonacci_iterative({}) = {}", n, fibonacci_iterative(n));
}
