use reqwest::{Client, header::{ETag, HeaderMap}};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let etag_map: HashMap<String, ETag> = HashMap::new();
    let url = "https://example.com";

    // Fetch the content
    let response = client.get(url)
        .send()
        .await?
        .headers().clone();

    // Check if ETag exists in headers
    if let Some(etag) = response.get::<ETag>() {
        println!("ETag: {:?}", etag);
    } else {
        println!("No ETag found!");
    }
    Ok(())
}