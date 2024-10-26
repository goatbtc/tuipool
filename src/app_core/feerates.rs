use reqwest::Error;
use serde_json::Value;

pub async fn get_recent_feerates() -> Result<String, Error> {
    let api_url = "https://mempool.space/api/v1/fees/recommended";
    let response = reqwest::get(api_url).await?.json::<Value>().await?;
    
    let fast = response["fastestFee"].as_f64().unwrap_or(15.0);
    let medium = response["halfHourFee"].as_f64().unwrap_or(10.0);
    let slow = response["hourFee"].as_f64().unwrap_or(5.0);
    let expulsion_threshold = response["minimumFee"].as_f64().unwrap_or(2.0);

    Ok(format!(
        "Feerates:\n\
        - Fast confirmation: {:.2} sat/vB\n\
        - Medium confirmation: {:.2} sat/vB\n\
        - Slow confirmation: {:.2} sat/vB\n\
        - Expulsion threshold: {:.2} sat/vB",
        fast, medium, slow, expulsion_threshold
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_recent_feerates() {
        match get_recent_feerates().await {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

