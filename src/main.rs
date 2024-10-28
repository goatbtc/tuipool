mod api;
mod app_core;
mod data;
mod ui;

use cursive::{Cursive, CursiveExt};
use std::error::Error;
use ui::menubar::setup_menubar;
use ui::blocks::*;
use ui::exit::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Our TUI
    let mut siv = Cursive::default();

    // Menubar setup
    setup_menubar(&mut siv);
    siv.set_autohide_menu(false);

    // Blocks view (initial load)
    blocks_view(&mut siv);

    // "q" to quit
    siv.add_global_callback('q', |s| show_exit_dialog(s));

    siv.run();

    Ok(())
}
