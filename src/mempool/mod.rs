use std::error::Error;
use reqwest::Client;
use serde::Deserialize;
use cursive::Cursive;
use cursive::views::Dialog;

/// API endpoints
const MEMPOOL_API_URL: &str = "https://mempool.space/api/mempool";
const FEE_ESTIMATES_URL: &str = "https://mempool.space/api/v1/fees/recommended";

/// Struct for general mempool data
#[derive(Deserialize, Debug)]
pub struct MempoolData {
    pub count: u64,
    pub vsize: u64,
}

/// Struct for fee estimate data
#[derive(Deserialize, Debug)]
pub struct FeeEstimates {
    pub fastest_fee: f64,
    pub half_hour_fee: f64,
    pub hour_fee: f64,
    pub minimum_fee: f64,
}

impl FeeEstimates {
    /// Fetches recommended fees from the mempool API
    pub async fn fetch_fees() -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let response = client
            .get(FEE_ESTIMATES_URL)
            .send()
            .await?
            .json::<FeeEstimates>()
            .await?;
        
        Ok(response)
    }
}

/// Fetches general data on the mempool
pub async fn fetch_mempool_data() -> Result<MempoolData, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(MEMPOOL_API_URL)
        .send()
        .await?
        .json::<MempoolData>()
        .await?;
    
    Ok(response)
}

/// Function to display mempool information within the Cursive TUI
pub async fn show_mempool_data(siv: &mut Cursive) {
    match fetch_mempool_data().await {
        Ok(data) => {
            siv.add_layer(Dialog::info(format!(
                "Total transactions in mempool: {}\nTotal mempool size (vsize): {} vBytes",
                data.count, data.vsize
            )));
        }
        Err(_) => {
            siv.add_layer(Dialog::info("Failed to fetch mempool data."));
        }
    }
}

