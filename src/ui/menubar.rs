// src/ui/menubar.rs

use cursive::{menu, Cursive, views::Dialog, event::Key};
use crate::ui::{fees, confirmations};
use tokio::runtime::Handle;

pub fn setup_menubar(siv: &mut Cursive, handle: Handle) {
    siv.menubar()
        .add_subtree(
            "Blocks",
            menu::Tree::new()
                .leaf("Blocks", |s| {
                    s.add_layer(Dialog::info("Blocks information will be displayed here."));
                }),
        )
        .add_subtree(
            "Tools", // New submenu for the additional functionalities
            menu::Tree::new()
                .leaf("Show Recommended Fees", move |s| {
                    fees::show_fee_recommendations(s, handle.clone());
                })
                .leaf("Estimate Confirmation Time", move |s| {
                    confirmations::show_estimate_confirmation_time(s, handle.clone());
                })
                .leaf("Check TXID in Mempool", move |s| {
                    confirmations::check_tx_in_mempool(s, handle.clone());
                }),
        )
        .add_subtree(
            "Help",
            menu::Tree::new()
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("About this application"));
                }),
        )
        .add_leaf("Quit", |s| s.quit()); // Updated Quit option

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}

