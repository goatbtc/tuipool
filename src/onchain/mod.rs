use std::sync::{Arc, Mutex};
use reqwest::Client;
use serde::Deserialize;
use cursive::Cursive;
use cursive::views::Dialog;
use crate::data::{BlockData, BlockStorage};
use crate::ui::blocks::{start_block_refresh, render_blocks};
use anyhow::{Result, bail};

/// API endpoint for retrieving block data
const BLOCKS_API_URL: &str = "https://mempool.space/api/blocks";

// Define a struct to hold detailed block information from the API
#[derive(Deserialize, Debug)]
pub struct ApiBlockData {
    pub id: String,
    pub height: u64,
    pub tx_count: u64,
    pub size: u64,
    pub weight: u64,
    pub fee: Option<f64>,  // Make `fee` optional to handle cases where it may be missing
    pub timestamp: u64,
    pub pool_name: Option<String>,
}

/// Fetches the latest block data from an external API with enhanced error handling.
async fn fetch_latest_blocks() -> Result<Vec<ApiBlockData>> {
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

/// Displays recent block data within the Cursive TUI
pub fn show_onchain_data(siv: &mut Cursive) {
    let cb_sink = siv.cb_sink().clone();
    let block_storage = Arc::new(Mutex::new(BlockStorage::new()));

    // Fetch the latest blocks asynchronously and update the UI
    let storage_clone = block_storage.clone();
    tokio::spawn(async move {
        match fetch_latest_blocks().await {
            Ok(api_blocks) => {
                let blocks: Vec<BlockData> = api_blocks.into_iter().take(6).map(|api_block| {
                    BlockData {
                        height: api_block.height,
                        sat_per_vbyte: api_block.fee.unwrap_or(0.0) / api_block.size as f64,
                        transactions: api_block.tx_count,
                        btc_amount: api_block.fee.unwrap_or(0.0) / 100_000_000.0,
                        time: format!("{} seconds ago", api_block.timestamp),
                        pool: api_block.pool_name.unwrap_or("Unknown".to_string()),
                    }
                }).collect();

                // Lock the storage, update blocks, and release the lock before sending to `cb_sink`
                {
                    let mut storage = storage_clone.lock().unwrap();
                    storage.clear_blocks();
                    for block in blocks.iter() {
                        storage.add_block(block.clone());
                    }
                }

                // Send update to the Cursive UI to display the blocks
                let _ = cb_sink.send(Box::new(move |s| {
                    let blocks_to_display = storage_clone.lock().unwrap().get_blocks().to_vec();
                    render_blocks(s, blocks_to_display);
                }));
            }
            Err(err) => {
                let _ = cb_sink.send(Box::new(move |s| {
                    s.add_layer(Dialog::info(format!("Failed to fetch block data: {:?}", err)));
                }));
            }
        }
    });

    // Initial load of blocks if no data is available
    start_block_refresh(siv, block_storage.clone());
}

