use anyhow::{bail, Result};
use cursive::views::Dialog;
use cursive::Cursive;
use reqwest::Client;
use serde::Deserialize;
use tokio::task;
use std::error::Error;
use std::sync::{Arc, Mutex};

const BLOCKS_API_URL: &str = "https://mempool.space/api/blocks";
const MEMPOOL_API_URL: &str = "https://mempool.space/api/mempool";
const FEE_ESTIMATES_URL: &str = "https://mempool.space/api/v1/fees/recommended";

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
