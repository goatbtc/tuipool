use crate::api::client::fetch_latest_blocks;
use crate::{BlockData, BlockStorage};
use cursive::views::{Dialog, LinearLayout, Panel, TextView};
use cursive::Cursive;

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
                        .map(|api_block| BlockData {
                            height: api_block.height,
                            sat_per_vbyte: api_block.fee.unwrap_or(0.0) / api_block.size as f64,
                            transactions: api_block.tx_count,
                            btc_amount: api_block.fee.unwrap_or(0.0) / 100_000_000.0,
                            time: format!("{} seconds ago", api_block.timestamp),
                            pool: api_block.pool_name.unwrap_or("Unknown".to_string()),
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
