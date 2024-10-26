use std::error::Error;
use reqwest::Client;
use serde::Deserialize;
use cursive::Cursive;
use cursive::views::Dialog;

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

/// Function to display blockchain information within the Cursive TUI
pub async fn show_onchain_data(siv: &mut Cursive) {
    match BlockchainInfo::fetch_info().await {
        Ok(info) => {
            siv.add_layer(Dialog::info(format!(
                "Current Block Height: {}\nCurrent Difficulty: {:.2}\nMedian Time: {}\nHash Rate: {:.2} TH/s\nBlockchain: {}",
                info.block_height, info.difficulty, info.median_time, info.hash_rate, info.chain
            )));
        }
        Err(_) => {
            siv.add_layer(Dialog::info("Failed to fetch on-chain data."));
        }
    }
}

