use reqwest::{ Client, Error };
use serde_json::{ json, Value };
use std::env;
use dotenv;

pub async fn call(url: &str, method: &str) -> Result<Value, Error> {
    dotenv::dotenv().ok();
    println!("Connect called");
    let address = env::var("ADDRESS").expect("ADDRESS must be set in .env file");
    
    let client = Client::new();

    let request_body = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": [address, "latest"],
        "id": 1
    });
    
    // Send request to Polygon API
    let response = client.post(url)
        .header("Content-Type", "application/json") // Specify content type as JSON
        .body(request_body.to_string()) // Set the JSON request body
        .send()
        .await?;
    println!("response {:?}", response);
    // Read response body as bytes
    let response_body_bytes = response.bytes().await?;

    // Parse response body bytes as JSON
    let response_json: Value = serde_json::from_slice(&response_body_bytes).expect("Unable to parse the response body bytes");

    Ok(response_json)
}