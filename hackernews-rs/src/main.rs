// Import crates
use clap::Parser;
use hackernews_rs::get_stories;
use hackernews_rs::QueryType::{BestStories, NewStories, TopStories};

#[derive(Parser, Debug)]
// add extended help
#[clap(
    version = "4.1.13",
    author = "Bryce Whitney",
    about = "A command-line tool that allows you to get stories from Hacker News",
    after_help = "Example: cargo run -- -q best -n 3"
)]

// Create struct to hold command-line arguments
struct Args {
    // Argument for the query type
    #[clap(short, long, default_value = "new")]
    query_type: String,

    // Argument for the number of results to return
    #[clap(short, long, default_value = "3")]
    num: usize,
}

#[tokio::main]
async fn main() {
    // Get command-line arguments
    let args: Args = Args::parse();

    // Get the query type
    let query_type = match args.query_type.as_str() {
        "top" => TopStories,
        "best" => BestStories,
        _ => NewStories,
    };

    // Get the stories
    get_stories(query_type, args.num).await;
}
