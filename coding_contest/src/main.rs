// Crates
use clap::Parser;
use coding_contest::get_competitions;

#[derive(Parser, Debug)]
#[clap(
    version = "4.2.1",
    author = "Bryce Whitney",
    about = "A command-line tool that scrapes coding contest websites for upcoming contests",
    after_help = "Example: cargo run -- --num 5"
)]

struct Args {
    // Argument for the number of results to return
    #[clap(short, long, default_value = "5")]
    num: usize,
}

#[tokio::main]
async fn main() {
    // Get command-line arguments
    let args: Args = Args::parse();

    // Get the competitions
    get_competitions(args.num).await;
}
