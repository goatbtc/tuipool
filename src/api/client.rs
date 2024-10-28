use anyhow::{bail, Result};
use cursive::Cursive;
use reqwest::Client;
use serde::Deserialize;
use tokio::task;
use std::error::Error;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use reqwest;

const BLOCKS_API_URL: &str = "https://mempool.space/api/blocks";
const MEMPOOL_API_URL: &str = "https://mempool.space/api/mempool";
const FEE_ESTIMATES_URL: &str = "https://mempool.space/api/v1/fees/recommended";
const MEMPOOL_URL: &str = "https://mempool.space/api/mempool";
const BLOCKSTREAM_FEE_URL: &str = "https://blockstream.info/api/fee-estimates";

// Define a struct to hold detailed block information from the API
#[derive(Deserialize, Debug)]
pub struct ApiBlockData {
    pub id: String,
    pub height: u64,
    pub tx_count: u32,
    pub size: u64,
    pub weight: u64,
    pub fee: Option<f64>, // Make `fee` optional to handle cases where it may be missing
    pub timestamp: u64,
    pub pool_name: Option<String>,
}
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


#[derive(Deserialize, Debug)]
struct FeeEstimatesBS {
    #[serde(rename = "1")]
    fastest_fee: f64,
    #[serde(rename = "3")]
    three_blocks_fee: f64,
    #[serde(rename = "6")]
    six_blocks_fee: f64,
}

/// Fetches the latest block data from an external API with enhanced error handling.
pub async fn fetch_latest_blocks() -> Result<Vec<ApiBlockData>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true) // Temporarily bypass SSL certs for testing
        .build()?;

    let response = client.get(BLOCKS_API_URL).send().await?;

    if !response.status().is_success() {
        bail!("Failed API call with status: {:?}", response.status());
    }

    let blocks = response.json::<Vec<ApiBlockData>>().await?;
    Ok(blocks)
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
pub fn show_mempool_data(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();

    // Cria uma nova tarefa assíncrona usando `tokio::spawn`
    task::spawn(async move {
        // Chama a função `fetch_mempool_data` para obter os dados
        match fetch_mempool_data().await {
            Ok(data) => {
                // Envia uma mensagem de atualização para a UI através do `cb_sink`
                let _ = cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.add_layer(Dialog::info(format!(
                        "Total transactions in mempool: {}\nTotal mempool size (vsize): {} vBytes",
                        data.count, data.vsize
                    )));
                }));
            }
            Err(_) => {
                // Em caso de erro, envia uma mensagem de erro para a UI
                let _ = cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.add_layer(Dialog::info("Failed to fetch mempool data."));
                }));
            }
        }
    });
}





pub fn setup_mempool_menu(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();

    siv.add_layer(
        Dialog::new()
            .title("Fee rates Information")
            .content(
                LinearLayout::vertical()
                    .child(TextView::new("Fetching recent fee rates...").with_name("fee_view"))
                    .child(TextView::new("Enter a txid to check:"))
                    .child(
                        EditView::new()
                            .on_submit({
                                let cb_sink = cb_sink.clone();
                                move |s, txid| {
                                    let txid_clone = txid.to_string();
                                    s.pop_layer();
                                    check_txid_in_mempool(cb_sink.clone(), txid_clone);
                                }
                            })
                            .with_name("txid_input")
                            .fixed_width(40),
                    ),
            )
            .button("Refresh Fees", {
                let cb_sink = cb_sink.clone();
                move |s| {
                    s.call_on_name("fee_view", |view: &mut TextView| {
                        view.set_content("Fetching latest fee rates...");
                    });
                    fetch_and_display_fees(cb_sink.clone());
                }
            })
            .button("Close", |s| {
                s.pop_layer();
            }),
    );

    fetch_and_display_fees(cb_sink);
}

fn fetch_and_display_fees(cb_sink: cursive::CbSink) {
    // Create an async block and spawn it on the Cursive event loop
    tokio::spawn(async move {
        match fetch_fee_rates().await {
            Ok(fee_data) => {
                let fee_info = format!(
                    "Fastest (1 block): {:.2} sat/vB\n3 blocks: {:.2} sat/vB\n6 blocks: {:.2} sat/vB",
                    fee_data.fastest_fee, fee_data.three_blocks_fee, fee_data.six_blocks_fee
                );
                let _ = cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.call_on_name("fee_view", |view: &mut TextView| {
                        view.set_content(fee_info);
                    });
                }));
            }
            Err(_) => {
                let _ = cb_sink.send(Box::new(move |siv: &mut Cursive| {
                    siv.call_on_name("fee_view", |view: &mut TextView| {
                        view.set_content("Failed to fetch fee data.");
                    });
                }));
            }
        }
    });
}

async fn fetch_fee_rates() -> Result<FeeEstimatesBS, Box<dyn Error>> {
    let response = reqwest::get(BLOCKSTREAM_FEE_URL).await?;
    let fee_data = response.json::<FeeEstimatesBS>().await?;
    Ok(fee_data)
}

fn check_txid_in_mempool(cb_sink: cursive::CbSink, txid: String) {
    // Spawn an async task for checking the txid in the mempool
    let cb_sink_clone = cb_sink.clone();
    tokio::spawn(async move {
        let url = format!("https://mempool.space/api/tx/{}", txid);
        let client = reqwest::Client::new();
        let dialog_message = match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => "Transaction is in the mempool!".to_string(),
            _ => "Transaction not found in the mempool.".to_string(),
        };

        let _ = cb_sink_clone.send(Box::new(move |siv: &mut Cursive| {
            siv.add_layer(Dialog::info(dialog_message));
        }));
    });
}
