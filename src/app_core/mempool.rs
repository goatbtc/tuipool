use reqwest::Error;

pub async fn check_transaction(txid: &str) -> Result<String, Error> {
    let api_url = format!("https://mempool.space/api/tx/{}/status", txid);

    let response = reqwest::get(&api_url).await?.json::<serde_json::Value>().await?;

    if response["confirmed"].as_bool().unwrap_or(false) {
        Ok(format!("Transaction {} has already been confirmed.", txid))
    } else {
        Ok(format!("Transaction {} is in the mempool and waiting for confirmation.", txid))
    }
}

