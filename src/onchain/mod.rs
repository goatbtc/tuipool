use std::error::Error;
use reqwest::Client;
use serde::Deserialize;

/// API endpoint for retrieving blockchain data
const BLOCKCHAIN_INFO_URL: &str = "https://mempool.space/api/blockchain";

// Define a struct to hold general blockchain information
#[derive(Deserialize, Debug)]
pub struct BlockchainInfo {
    pub block_height: u64,
    pub difficulty: f64,
    pub median_time: u64,
    pub hash_rate: f64,
    pub chain: String,
}

impl BlockchainInfo {
    /// Fetches general blockchain data
    pub async fn fetch_info() -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let response = client
            .get(BLOCKCHAIN_INFO_URL)
            .send()
            .await?
            .json::<BlockchainInfo>()
            .await?;
        
        Ok(response)
    }
}

/// Function to display basic blockchain information
pub async fn display_blockchain_info() -> Result<(), Box<dyn Error>> {
    match BlockchainInfo::fetch_info().await {
        Ok(info) => {
            println!("Current Block Height: {}", info.block_height);
            println!("Current Difficulty: {}", info.difficulty);
            println!("Median Time: {}", info.median_time);
            println!("Hash Rate: {}", info.hash_rate);
            println!("Blockchain: {}", info.chain);
        }
        Err(e) => {
            eprintln!("Failed to fetch blockchain info: {}", e);
        }
    }
    Ok(())
}

