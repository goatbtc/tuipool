use std::sync::{Arc, Mutex};
use cursive::views::{Dialog, LinearLayout, Panel, TextView};
use cursive::Cursive;
use reqwest::Client;
use serde::Deserialize;
use crate::BlockStorage;
use crate::data::BlockData;

/// Creates a view for an individual block, formatted with its details
pub fn create_block_view(block: BlockData) -> Panel<TextView> {
    let block_info = format!(
        "Height: {}\n{:.2} sat/vB\n{} transactions\n{:.8} BTC\n{}",
        block.height, block.sat_per_vbyte, block.transactions, block.btc_amount, block.time
    );

    Panel::new(TextView::new(block_info)).title(format!("Block {}", block.height))
}

/// Renders a list of blocks horizontally within a dialog
pub fn render_blocks(siv: &mut Cursive, blocks: Vec<BlockData>) {
    let mut layout = LinearLayout::horizontal();

    for block in blocks {
        layout.add_child(create_block_view(block));
    }

    siv.add_layer(Dialog::around(layout).title("Onchain"));
}

/// Refreshes block data by generating new blocks and displays them in the UI
pub fn start_block_refresh(siv: &mut Cursive, block_storage: Arc<Mutex<BlockStorage>>) {
    let cb_sink = siv.cb_sink().clone();

    tokio::spawn(async move {
        match fetch_latest_blocks().await {
            Ok(blocks) => {
                let mut storage = block_storage.lock().unwrap();
                storage.clear_blocks();

                for block in blocks.into_iter().take(6) {
                    storage.add_block(BlockData {
                        height: block.height,
                        sat_per_vbyte: block.fee / block.size as f64,
                        transactions: block.tx_count,
                        btc_amount: block.fee / 100_000_000.0,
                        time: format!("{} seconds ago", block.timestamp),
                        pool: block.pool_name.unwrap_or("Unknown".to_string()),
                    });
                }

                // Send update to the Cursive UI to display the blocks
                let blocks_clone = storage.get_blocks().to_vec();
                let _ = cb_sink.send(Box::new(move |s| {
                    render_blocks(s, blocks_clone);
                }));
            }
            Err(_) => {
                let _ = cb_sink.send(Box::new(move |s| {
                    s.add_layer(Dialog::info("Failed to fetch block data."));
                }));
            }
        }
    });
}

/// Fetches the latest block data from an external API
async fn fetch_latest_blocks() -> Result<Vec<ApiBlockData>, reqwest::Error> {
    let client = Client::new();
    let url = "https://mempool.space/api/blocks";
    let response = client.get(url).send().await?;
    let blocks = response.json::<Vec<ApiBlockData>>().await?;
    Ok(blocks)
}

#[derive(Deserialize, Debug, Clone)]
struct ApiBlockData {
    id: String,
    height: u64,
    tx_count: u64,
    size: u64,
    weight: u64,
    fee: f64,
    timestamp: u64,
    pool_name: Option<String>,
}

