mod cli;
mod ui;
mod data;
mod app_core;
pub mod onchain;
pub mod mempool;

use std::error::Error;
use std::sync::{Arc, Mutex};
use cursive::{Cursive, CursiveExt};
use cursive::views::{Dialog, LinearLayout};
use ui::{blocks, exit, menubar::setup_menubar};
use data::{BlockStorage, BlockData};
use cursive::view::Nameable; // Import Nameable trait for `.with_name()`

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Our TUI
    let mut siv = Cursive::default();

    // Menubar setup
    setup_menubar(&mut siv);
    siv.set_autohide_menu(false);

    // Shared storage for blocks
    let block_storage = Arc::new(Mutex::new(BlockStorage::new()));

    // Fetch the latest blocks asynchronously and update the UI
    let cb_sink = siv.cb_sink().clone();
    tokio::spawn({
        let cb_sink = cb_sink.clone();
        async move {
            match onchain::fetch_latest_blocks().await {
                Ok(api_blocks) => {
                    // Convert `ApiBlockData` to `BlockData` and limit to 6 blocks
                    let blocks: Vec<_> = api_blocks.into_iter().take(6).map(|api_block| BlockData {
                        height: api_block.height,
                        sat_per_vbyte: api_block.fee.unwrap_or(0.0) / api_block.size as f64,
                        transactions: api_block.tx_count,
                        btc_amount: api_block.fee.unwrap_or(0.0) / 100_000_000.0,
                        time: format!("{} seconds ago", api_block.timestamp),
                        pool: api_block.pool_name.unwrap_or("Unknown".to_string()),
                    }).collect();

                    let _ = cb_sink.send(Box::new(move |s| {
                        blocks::render_blocks(s, blocks);
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

    // Directly render blocks on the main screen without an introductory window
    siv.add_layer(
        LinearLayout::vertical()
            .child(Dialog::new().title("Onchain").content(LinearLayout::horizontal()))
    );

    // "q" to quit
    siv.add_global_callback('q', |s| exit::show_exit_dialog(s));

    siv.run();

    Ok(())
}

