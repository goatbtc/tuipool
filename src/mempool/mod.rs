use std::error::Error;
use reqwest::Client;
use serde::Deserialize;

/// API endpoint for retrieving mempool data
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

/// Displays basic mempool information
pub async fn display_mempool_info() -> Result<(), Box<dyn Error>> {
    match fetch_mempool_data().await {
        Ok(data) => {
            println!("Total transactions in mempool: {}", data.count);
            println!("Total mempool size (vsize): {} vBytes", data.vsize);
        }
        Err(e) => {
            eprintln!("Failed to fetch mempool data: {}", e);
        }
    }
    Ok(())
}

/// Displays recommended transaction fees
pub async fn display_fee_estimates() -> Result<(), Box<dyn Error>> {
    match FeeEstimates::fetch_fees().await {
        Ok(fees) => {
            println!("Recommended Fees:");
            println!(" - Fastest confirmation: {:.2} sat/vB", fees.fastest_fee);
            println!(" - Half-hour confirmation: {:.2} sat/vB", fees.half_hour_fee);
            println!(" - Hour confirmation: {:.2} sat/vB", fees.hour_fee);
            println!(" - Minimum fee: {:.2} sat/vB", fees.minimum_fee);
        }
        Err(e) => {
            eprintln!("Failed to fetch fee estimates: {}", e);
        }
    }
    Ok(())
}

