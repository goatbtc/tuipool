mod cli;
mod ui;
mod data;

use std::error::Error;
use cursive::{Cursive, CursiveExt};
use ui::{blocks,exit,fees,menubar::{self, setup_menubar}};
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
    let mut block_storage = BlockStorage::new();
    blocks::start_block_refresh(&mut siv, &mut block_storage);

    // "q" to quit
    siv.add_global_callback('q', |s| exit::show_exit_dialog(s));

    siv.run();

    Ok(())
}

