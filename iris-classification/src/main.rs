use clap::Parser;

#[derive(Parser, Debug)]
// add extended help
#[clap(
    version = "4.1.8",
    author = "Bryce Whitney",
    about = "A command-line tool that compares the accuracy of two decision tree models",
    after_help = "Example: cargo run -- --random_seed 42"
)]

// Create struct to hold command-line arguments
struct Args {
    // Create argument to hold sentence
    #[clap(short, long, default_value = "1")]
    random_seed: u64,
}

fn main() {
    // Get command-line arguments
    let args = Args::parse();
    // Get the accuracy for each model
    let entropy_accuracy = iris_classification::entropy_model(args.random_seed);
    let gini_accuracy = iris_classification::gini_model(args.random_seed + 1); // Add 1 to the random seed to get different results

    // Print the accuracy for each model
    println!("Accuracy for Entropy Model: {}", entropy_accuracy);
    println!("Accuracy for Gini Model: {}", gini_accuracy);
    
    // Winner is whichever model has the highest accuracy
    if entropy_accuracy > gini_accuracy {
        println!("Entropy Model is the winner!");
    } else if gini_accuracy > entropy_accuracy {
        println!("Gini Model is the winner!");
    } else {
        println!("Both models are equally good!");
    }
}
