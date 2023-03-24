// Import crates
//use reqwest;
//use serde;

// Constants
const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

// Enums
pub enum QueryType {
    TopStories,
    BestStories,
    NewStories,
    JobStories,
}

// Functions
// Make the proper request to the API
pub async fn get_stories(query_type: QueryType) {
    let url = format!("{}/{}", BASE_URL, get_extension(query_type));
    
    // TODO: Delete later after testing
    println!("URL: {}", url);
}

// Get the extension depending on the QueryType
// ? Do we need .json at the end of the extension?
fn get_extension(query_type: QueryType) -> &'static str {
    match query_type {
        QueryType::TopStories => "topstories.json",
        QueryType::BestStories => "beststories.json",
        QueryType::NewStories => "newstories.json",
        QueryType::JobStories => "jobstories.json",
    }
}
