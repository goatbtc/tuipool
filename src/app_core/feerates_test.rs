pub async fn get_recent_feerates() -> Result<String, Error> {
    let api_url = "https://mempool.space/api/v1/fees/recommended";
    let response = reqwest::get(api_url).await?.json::<serde_json::Value>().await?;
    println!("Raw API response: {:?}", response);  // Print raw response for verification
    
    // Continue with processing
    ...
}

