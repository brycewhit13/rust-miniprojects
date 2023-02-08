use clap::Parser;
use tokio;

#[derive(Parser, Debug)]
//add extended help
#[clap(
    version = "4.1.4",
    author = "Bryce Whitney",
    about = "A command-line tool that looks at conversion rates between currencies",
    after_help = "Example: cargo run -- "
)]
struct Args {
    // First currency
    #[arg(short, long)]
    origin: String,

    // Second currency
    #[arg(short, long)]
    destination: String,
}

// Find a list of currency codes here: https://www.iban.com/currency-codes
#[tokio::main]
async fn main() {
    let args = Args::parse();
    currency_converter::get_conversion_rate(&args.origin, &args.destination).await;
}
