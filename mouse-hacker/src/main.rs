use clap::Parser;
use std::thread;

// TODO: Where to add help

#[derive(Parser, Debug)]
//add extended help
#[clap(
    version = "4.1.4",
    author = "Bryce Whitney",
    about = "A command-line tool that controls your mouse for a brief period of time",
    after_help = "Example: cargo run -- --delay 5 --loops 3"
)]

struct Args {
    /// Delay in seconds
    #[clap(short, long, default_value = "1")]
    delay: String,

    /// Number of times to loop through the motion
    #[clap(short, long, default_value = "3")]
    loops: String,
}

fn main() {
    // get the delay argument
    let args = Args::parse();

    // Max of ten loops
    let mut num_loops = args.loops.parse::<u32>().unwrap();
    if num_loops >= 10 {
        println!("Max of ten loops! Setting the number of loops to 10");
        num_loops = 10;
    }

    // Loop through the motions
    for _ in 0..num_loops {
        thread::sleep(std::time::Duration::from_secs(
            args.delay.parse::<u64>().unwrap(),
        ));
        mouse_hacker::traingle_movement();
    }
}
