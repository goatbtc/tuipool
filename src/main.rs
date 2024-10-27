mod cli;
mod ui;
mod data;
mod app_core;
pub mod onchain;
pub mod mempool;
use std::error::Error;
use std::sync::{Arc, Mutex};
use cursive::{Cursive, CursiveExt};
use ui::{blocks, exit, menubar::setup_menubar};

// src/main.rs
use crate::data::BlockStorage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // our TUI
    let mut siv = Cursive::default();

    // menubar 
    setup_menubar(&mut siv);
    siv.set_autohide_menu(false);

    // blocks view
    let block_storage = Arc::new(Mutex::new(BlockStorage::new()));
    blocks::start_block_refresh(&mut siv, block_storage.clone());

    // "q" to quit
    siv.add_global_callback('q', |s| exit::show_exit_dialog(s));

    siv.run();

    Ok(())
}

