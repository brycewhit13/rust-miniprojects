// Import clap
use clap::Parser;

#[derive(Parser, Debug)]
// add extended help
#[clap(
    version = "4.1.1",
    author = "Bryce Whitney",
    about = "A command-line tool that translates English to Pig Latin",
    after_help = "Example: cargo run -- --sentence \"hello world!\""
)]

// Create struct to hold command-line arguments
struct Args {
    // Create argument to hold sentence
    #[clap(short, long, default_value = "")]
    sentence: String,
}

fn main() {
    // Get command-line arguments
    let args = Args::parse();

    // Check if string passed is empty
    if args.sentence.is_empty() {
        // Print error message
        println!("Error: No sentence passed!");
        // Exit program
        std::process::exit(1);
    } else {
        // convert sentence to pig latin
        pig_latin::sent_to_pig_latin(&args.sentence);
    }
}
