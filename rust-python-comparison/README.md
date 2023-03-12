# rust-python-comparison

## Description

The goal of this project is to provide some analysis of the speed difference between Rust (a compiled language) and Python (an interpreted language). Currently there are two tasks that are performed for each, with hopes to add more to the future.

1. `count_to_billion`: Counts from 0 to 1 billion
2. `load_airport_data`: Loads this [airport data](https://openflights.org/data.html) which consists of ~7.7k rows.
    - This is accomplished with **pandas** in python and **polars** in rust.

## How to run

There is both Rust and Python code that can be run in this repo. Follow these steps to run everything from start to finish:

1. cd into `src`
2. Run the Rust benchmarks with `cargo run`
3. Run the Python benchmarks with `python main.py`
    - If you need to install the requirements, run `pip install -r requirements.txt`

## Results

Here are the current results for each language. Note that this was run locally so the actual numbers may differ depending on what machine you run the benchmark on.

||Python|Rust|
|---|---|---|
|count_to_billion()|100125 ms|11423 ms|
|load_airport_data()|63 ms|129 ms|

## Further Improvements:

    1. Add more tasks for comparison
    2. Allow user to choose individual tasks to compare in the command-line
    3. I recognize this is not a perfect comparison, and have room to improve on making the code more similar