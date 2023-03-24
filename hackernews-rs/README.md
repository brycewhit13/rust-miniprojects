# Hacker News Retrieval

## Description

This CLI allows you to get articles from [Hacker News](https://news.ycombinator.com/?p=2) straight from your terminal. This is accomplished using the free [Hacker News API](https://github.com/HackerNews/API). It is completely free and doesn't require any authentication, so you can try it out yourself right away. This programs makes a request to the API and returns the titles and URLs of the articles. The user has the ability to indicate both the type and number of articles returned which is detailed below.

## How to run

1. cd into the directory with `cd hackernews-rs`
2. Depending on your preferences, you will run one of the following commands. Please see the next section for more information about the arguments you can pass.
    - For default behavior: `cargo run`
    - To specify the type of article: `cargo run -- -q [QUERY_TYPE]`
    - To specify the number of results returned: `cargo run -- -n [NUM_RESULTS]`
    - To specify everything: `cargo run -- -q [QUERY_TYPE] -n [NUM_RESULTS]`

## Arguments

### Query Type

You can specify what articles you want the CLI to return from Hacker News. Currently there are three options available that you can pass using the `--query_type` option (`-q` for short). You can look at the newest stories, top stories, or the best stories. By default the program will return the newest articles. 
    - For the best stories pass `best` to the query type argument
    - For the top stories pass `top` to the query type argument
    - For the newest stories pass `new` to the query type argument

### Number of Results

You can specify the number of results that you want returned with the `--num_results` argument (`-n` for short). This should be a numerical value and if it is too large, you run the risk of causing the program to crash due to an error. By default it is set to 3 articles. The maximium number is 500.