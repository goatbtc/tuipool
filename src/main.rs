mod app_core;
mod cli;
mod data;
mod ui;
mod api;

use crate::data::data::{BlockData, BlockStorage};
use cursive::{Cursive, CursiveExt};
use ui::blocks::blocks_view;
use std::error::Error;
use ui::{exit, menubar::setup_menubar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Our TUI
    let mut siv = Cursive::default();

    // Menubar setup
    setup_menubar(&mut siv);
    siv.set_autohide_menu(false);

    // Blocks view
    blocks_view(&mut siv);

    // "q" to quit
    siv.add_global_callback('q', |s| exit::show_exit_dialog(s));

    siv.run();

    Ok(())
}
