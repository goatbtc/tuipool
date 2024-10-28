use crate::{BlockData, BlockStorage};
use cursive::views::{Dialog, LinearLayout, Panel, TextView};
use cursive::Cursive;
use std::sync::{Arc, Mutex};

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

// pub fn start_block_refresh(siv: &mut Cursive, block_storage: Arc<Mutex<BlockStorage>>) {
// }

