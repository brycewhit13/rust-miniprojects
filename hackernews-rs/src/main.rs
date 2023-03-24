// Import crates
use hackernews_rs::get_stories;
use hackernews_rs::QueryType::{NewStories, TopStories,  BestStories, JobStories};
use tokio;
use clap::Parser;

#[derive(Parser, Debug)]
// add extended help
// TODO: Add an example after the help
#[clap(
    version = "4.1.13",
    author = "Bryce Whitney",
    about = "A command-line tool that allows you to get stories from Hacker News",
    after_help = "Example: cargo run -- -q best"
)]

// Create struct to hold command-line arguments
struct Args {
    // Create argument to hold sentence
    #[clap(short, long, default_value = "new")]
    query_type: String,
}

#[tokio::main]
async fn main() {
    // Get command-line arguments
    let args: Args = Args::parse();

    // Get the query type
    let query_type = match args.query_type.as_str() {
        "top" => TopStories,
        "best" => BestStories,
        "new" => NewStories,
        "job" => JobStories,
        _ => NewStories,
    };

    // Get the stories
    get_stories(query_type).await;
}
