// Constants
const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

// Enums
pub enum QueryType {
    TopStories,
    BestStories,
    NewStories,
}

// Functions
// Make the proper request to the API
pub async fn get_stories(query_type: QueryType, num_stories: usize) {
    // Make the request for the story ids
    let id_url = format!("{}/{}", BASE_URL, get_extension(query_type));
    let story_ids = reqwest::get(id_url)
        .await
        .expect("REASON")
        .text()
        .await
        .unwrap();

    // Convert the story_ids to a vector of u32
    let story_ids = serde_json::from_str::<Vec<u32>>(&story_ids).unwrap();

    // Make the request for the story link
    for (i, _id) in story_ids.iter().enumerate().take(num_stories) {
        let story_url = format!("{}/item/{}.json", BASE_URL, story_ids[i]);
        let story_info = reqwest::get(story_url)
            .await
            .expect("REASON")
            .text()
            .await
            .unwrap();
        let story_info_json = serde_json::from_str::<serde_json::Value>(&story_info).unwrap();

        // Get info from the json
        let title = story_info_json["title"].as_str().unwrap();
        let url = story_info_json["url"].as_str().unwrap();

        // Print the results
        println!("Article Title: {}", title);
        println!("URL: {}", url);
        if i != num_stories - 1 {
            println!("\n");
        }
    }
}

// Get the extension depending on the QueryType
// ? Do we need .json at the end of the extension?
fn get_extension(query_type: QueryType) -> &'static str {
    match query_type {
        QueryType::TopStories => "topstories.json",
        QueryType::BestStories => "beststories.json",
        QueryType::NewStories => "newstories.json",
    }
}
