// Crates
use clap::Parser;
use int_conversion::{int_to_binary, int_to_hex, int_to_octal};

#[derive(Parser, Debug)]
// add extended help
#[clap(
    version = "4.2.1",
    author = "Bryce Whitney",
    about = "A command-line tool that converts int to binary, hex, and octal.",
    after_help = "Example: cargo run -- -i 10 -b -h -o"
)]

// Create struct to hold command-line arguments
struct Args {
    // Argument for the integer argument
    #[clap(short, long, default_value = "15")]
    int: u32,

    // Argument for the binary argument
    #[clap(short, long, default_value = "false")]
    binary: bool,

    // Argument for the hex argument
    #[clap(short, long, default_value = "false")]
    n_hex: bool,

    // Argument for the octal argument
    #[clap(short, long, default_value = "false")]
    octal: bool,
}


fn main() {
    // Parse the command-line arguments
    let args = Args::parse();
    
    if args.binary{
        println!("{} is {} in binary.", args.int, int_to_binary(args.int));
    }

    if args.n_hex{
        println!("{} is {} in hex.", args.int, int_to_hex(args.int));
    }

    if args.octal{
        println!("{} is {} in octal.", args.int, int_to_octal(args.int));
    }

    if !args.binary && !args.n_hex && !args.octal{
        println!("No conversion type specified. Please specify a conversion type!")
    }
}
