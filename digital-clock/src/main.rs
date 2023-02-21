// Crates
use clap::Parser;

#[derive(Parser, Debug)]
//add extended help
#[clap(
    version = "4.1.4",
    author = "Bryce Whitney",
    about = "A command-line tool that acts as a digitcal clock",
    after_help = "Example: cargo run "
)]
struct Args {
}

fn main() {
    // print an empty line
    println!();

    // run the proper main loop
    digital_clock::main_loop();
}
