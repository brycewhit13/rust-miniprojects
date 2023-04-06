// Crates
use reqwest;
use serde_json;


pub async fn get_competitions(items_to_show: usize){
    // Get the competitions
    let url = "https://kontests.net/api/v1/all";

    // Get the response
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();
    let body_json = serde_json::from_str::<serde_json::Value>(&body).unwrap();

    // Format the response
    format_response(body_json, items_to_show);
}

pub fn format_response(body_json: serde_json::Value, mut items_to_show: usize) {
    // Determine how many items there are
    let num_items = body_json.as_array().unwrap().len();
    if items_to_show > num_items {
        println!("There are only {} items to show", num_items);
        items_to_show = num_items;
    }

    // Loop through each item
    for i in 0..items_to_show {
        // Extract the necessary information
        let name = body_json[i]["name"].as_str().unwrap();
        let url = body_json[i]["url"].as_str().unwrap();
        let mut start_time = body_json[i]["start_time"].as_str().unwrap();
        let mut end_time = body_json[i]["end_time"].as_str().unwrap();
        let site = body_json[i]["site"].as_str().unwrap();
        let status = body_json[i]["status"].as_str().unwrap();

        // Extract the date from the start and end time
        let start_date = start_time.split("T").collect::<Vec<&str>>()[0];
        start_time = start_time.split("T").collect::<Vec<&str>>()[1];

        let end_date = end_time.split("T").collect::<Vec<&str>>()[0];
        end_time = end_time.split("T").collect::<Vec<&str>>()[1];

        // Reformat the date to be MM/DD/YYYY
        let start_date_format = format!("{}/{}/{}", start_date.split("-").collect::<Vec<&str>>()[1], start_date.split("-").collect::<Vec<&str>>()[2], start_date.split("-").collect::<Vec<&str>>()[0]);
        let end_date_format = format!("{}/{}/{}", end_date.split("-").collect::<Vec<&str>>()[1], end_date.split("-").collect::<Vec<&str>>()[2], end_date.split("-").collect::<Vec<&str>>()[0]);

        // Convert the time to EST
        let start_time = format!("{:02}:{:02} EST", start_time.split(":").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() - 4, start_time.split(":").collect::<Vec<&str>>()[1]);
        let end_time = format!("{:02}:{:02} EST", end_time.split(":").collect::<Vec<&str>>()[0].parse::<i32>().unwrap() - 4, end_time.split(":").collect::<Vec<&str>>()[1]);

        // Print the results
        println!("Name: {}", name);
        println!("URL: {}", url);
        println!("Start Date: {}", start_date_format);
        println!("Start Time: {}", start_time);
        println!("End Date: {}", end_date_format);
        println!("End Time: {}", end_time);
        println!("Site: {}", site);
        println!("Status: {}", status);
        if i != items_to_show - 1 {
            println!("\n");
        }
    }
}