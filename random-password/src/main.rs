//Command-line tool to get rust project ideas
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "4.0.32",
    author = "Bryce Whitney",
    about = "A command-line tool that generates a random password.",
    after_help = "Example: cargo run --length 10"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(about = "Get a popular Rust crate")]
    Length {
        #[clap(short, long, default_value = "12")]
        characters: Option<usize>, // Determines the length of the password
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Length { characters }) => {
            // Minimum password is 3 characters long
            if characters.unwrap() < 3 {
                println!("Password must be at least three characters. Defaulting to a 3 character password!");
                let password: String = random_password::generate_password(3);
                println!("Password: {}", password);
            }
            else{
                let password: String = random_password::generate_password(characters.unwrap());
                println!("Password: {}", password);
            }
        }

        None => {
            let password: String = random_password::generate_password(12);
            println!("Password: {}", password);
        }
    }
}