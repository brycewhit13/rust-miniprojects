# mouse-hacker

## Motivation

This is a mini-project that takes control of your mouse and moves it in a triangular motion. The sole purpose of this project is to gain experience with ways to control the mouse and button clicks in rust. It will likely not be a very useful program to use outside of this task. 

I used the [autopilot crate](https://crates.io/crates/autopilot) to accomplish this, as well as the sleeping functionality from threads in the standard rust crate. The CLI is implemented using the [clap crate](https://crates.io/crates/clap). The code is very simple so feel free to take a look if you have any questions.

## How to run the program

There are 2 parameters the user can define when running the program: `delay` and `loops`. Delay controls the amount of time in seconds that the program waits between each time it moves the mouse in a triangular motion. On the other hand, `loops` controls how many times the program will run. If there are 3 loops, that means the mouse will move in a triangle three times. By default, `delay=1` and `loops=3`. 

To run the program, do the following:
1) cd into `mouse-hacker` if not already there
2) Run the command `cargo run` for default implementation or `cargo run -- --delay [DELAY_IN_SECONDS] --loops [NUMBER_OF_LOOPS]` to define your own parameters


If you have questions, run the following command for help: `cargo run -- --help`.