use crate::api::client::fetch_latest_blocks;
use crate::data::data::{BlockData, BlockStorage};
use cursive::views::{Dialog, LinearLayout, Panel, TextView};
use cursive::Cursive;
use std::time::{SystemTime, UNIX_EPOCH};


/// Creates a view for an individual block, formatted with its details
pub fn create_block_view(block: BlockData) -> Panel<TextView> {
    let block_info = format!(
        "Height: {}\n{} sat/vB\n{} transactions\n{} BTC\n{}",
        block.height,
        if block.sat_per_vbyte.is_nan() {
            "N/A".to_string()
        } else {
            format!("{:.2}", block.sat_per_vbyte)
        },
        block.transactions,
        if block.btc_amount.is_nan() {
            "N/A".to_string()
        } else {
            format!("{:.8}", block.btc_amount)
        },
        block.time
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

pub fn blocks_view(siv: &mut Cursive) {
    // Fetch the latest blocks asynchronously and update the UI
    let cb_sink = siv.cb_sink().clone();
    tokio::spawn({
        let cb_sink = cb_sink.clone();
        async move {
            match fetch_latest_blocks().await {
                Ok(api_blocks) => {
                    let blocks: Vec<BlockData> = api_blocks
                        .into_iter()
                        .take(6)
                        .map(|api_block| {
                            let sat_per_vbyte = match (api_block.fee, api_block.size) {
                                (Some(fee), size) if size > 0 => fee / size as f64,
                                _ => 0.0,
                            };

                            let btc_amount = match api_block.fee {
                                Some(fee) => fee / 100_000_000.0,
                                None => 0.0,
                            };

                            let time_diff_in_seconds = get_time_difference_in_seconds(api_block.timestamp);


                            BlockData {
                                height: api_block.height,
                                sat_per_vbyte,
                                transactions: api_block.tx_count,
                                btc_amount,
                                time: format_time_ago(time_diff_in_seconds),
                                pool: api_block.pool_name.unwrap_or("Unknown".to_string()),
                            }
                        })
                        .collect();
                    let _ = cb_sink.send(Box::new(move |s| {
                        render_blocks(s, blocks);
                    }));
                }
                Err(_) => {
                    let _ = cb_sink.send(Box::new(move |s| {
                        s.add_layer(Dialog::info("Failed to fetch block data."));
                    }));
                }
            }
        }
    });
}

fn format_time_ago(seconds_ago: u64) -> String {
    let minutes_ago = seconds_ago / 60;
    format!("{} minutes ago", minutes_ago)
}

fn get_time_difference_in_seconds(block_timestamp: u64) -> u64 {
    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    current_timestamp.saturating_sub(block_timestamp)
}
