/* Takes 2 currency codes and prints out the exchange rates */
use reqwest;
use std::env;
use std::fs;

pub async fn get_conversion_rate(code1: &str, code2: &str) {
    // Get the API key
    let key_path = env::current_dir().unwrap();
    let key_path_inter = key_path.join("credentials.cfg");
    let key_path_final = key_path_inter.to_str().unwrap();

    let file_string = fs::read_to_string(key_path_final).unwrap();
    let start_pos = file_string.find("\"").unwrap() + 1;
    let api_key = &file_string.trim()[start_pos..start_pos + 20];

    // Make the call to the API
    let api_call_str: String = format!(
        "https://free.currconv.com/api/v7/convert?q={code1}_{code2}&compact=ultra&apiKey={api_key}"
    );
    let result = reqwest::get(api_call_str)
        .await
        .expect("REASON")
        .text()
        .await
        .unwrap();

    // Format the results
    let start_pos = result.find(":").unwrap() + 0; // Convert to int
    let end_pos = start_pos + 6;
    let final_results = &result[start_pos..end_pos];
    println!(
        "Conversion Rate from {code1} to {code2}: {:?}",
        final_results
    );
}
