// Crates
use clap::Parser;
use health_classification::{load_data, split_data};

#[derive(Parser, Debug)]
// add extended help
#[clap(
    version = "4.2.1",
    author = "Bryce Whitney",
    about = "A command-line tool that allows you to train a model predicting the health of a fetus",
    after_help = "Example: cargo run -- -t 0.8 -s true -r 42"
)]

// Create struct to hold command-line arguments
struct Args {
    // Argument for the test set size
    #[clap(short, long, default_value = "0.8")]
    test_size: f64,

    // Argument for whether the data is shuffled
    #[clap(short, long, default_value = "true")]
    shuffle: bool,

    // Argument for the number of results to return
    #[clap(short, long, default_value = "42")]
    random_seed: u64,
}

fn main() {
    // Get command-line arguments
    let args: Args = Args::parse();

    // Load the data
    let health_df = load_data();
    //println!("Data: {}", health_df);

    // Split the data
    let (xtrain, xtest, ytrain, ytest) = split_data(&health_df, args.test_size, args.shuffle, args.random_seed);

    // Print the shape of each
    println!("xtrain: {:?}", xtrain.shape());
    println!("xtest: {:?}", xtest.shape());
    println!("ytrain: {:?}", ytrain.len());
    println!("ytest: {:?}", ytest.len());


}

