use cursive::views::Dialog;
use cursive::Cursive;
use reqwest::Error;
use serde_json::Value;

/// Fetches recent fee rates from the mempool API
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

/// Displays the recent fee rates in a Cursive dialog
pub fn show_average_fees(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();
    tokio::spawn(async move {
        match get_recent_feerates().await {
            Ok(fees) => {
                let _ = cb_sink.send(Box::new(move |s| {
                    s.add_layer(Dialog::info(fees).title("Average Fees"));
                }));
            }
            Err(_) => {
                let _ = cb_sink.send(Box::new(move |s| {
                    s.add_layer(Dialog::info("Failed to fetch fee data."));
                }));
            }
        }
    });
}
