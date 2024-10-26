pub mod blocks;
pub mod exit;
pub mod menubar;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

// Placeholder for Onchain data display
pub fn show_onchain_data(siv: &mut Cursive) {
    siv.add_layer(Dialog::around(TextView::new("Onchain data display")).title("Onchain Data"));
}

// Placeholder for Mempool data display
pub fn show_mempool_data(siv: &mut Cursive) {
    siv.add_layer(Dialog::around(TextView::new("Mempool data display")).title("Mempool Data"));
}

