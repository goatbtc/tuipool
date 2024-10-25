use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FeeRecommendations {
    pub fastestFee: f64,
    pub halfHourFee: f64,
    pub hourFee: f64,
    pub economyFee: f64,
    pub minimumFee: f64,
    pub expulsion_threshold: f64,
}

pub async fn get_fee_recommendations() -> Result<FeeRecommendations, Box<dyn std::error::Error>> {
    let response = reqwest::get("http://localhost:8000/recommend_fees")
        .await?
        .json::<FeeRecommendations>()
        .await?;
    Ok(response)
}

#[derive(Deserialize, Debug)]
pub struct ConfirmationEstimation {
    pub estimated_time_minutes: u32,
    pub estimated_blocks: u32,
}

pub async fn estimate_confirmation_time(fee_per_vsize: f64) -> Result<ConfirmationEstimation, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8000/estimate_confirmation_time")
        .query(&[("fee_per_vsize", fee_per_vsize)])
        .send()
        .await?
        .json::<ConfirmationEstimation>()
        .await?;
    Ok(response)
}

pub async fn is_tx_in_mempool(txid: String) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get("http://localhost:8000/is_tx_in_mempool")
        .query(&[("txid", txid)])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(response["in_mempool"].as_bool().unwrap_or(false))
}

