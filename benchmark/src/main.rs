use rand::{thread_rng, Rng};
use reqwest;
use std::{collections::HashMap, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL of the server
    let url = "http://nginx:8888/post";

    // Create a client to send requests
    let client = reqwest::Client::new();

    // Loop to send requests
    loop {
        let random_number = thread_rng().gen_range(1..=100);
        let mut d = HashMap::new();
        d.insert("data", random_number);
        // Send POST request
        let response = client.post(url).json(&d).send().await?;

        // Print response status
        println!("Response Status: {}", response.status());

        // Delay for 1 second before sending the next request
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
