mod cli;
mod ui;
mod data;
mod api_client; // Adicionado se ainda não estiver presente

use std::error::Error;
use cursive::{Cursive, CursiveExt};
//use data::data::BlockStorage;
use ui::{blocks, exit, fees, menubar::{self, setup_menubar}};
use tokio::runtime::Handle; // Importação necessária

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // our TUI
    let mut siv = Cursive::default();

    // Obter o Handle do runtime
    let handle = tokio::runtime::Handle::current();

    // menubar 
    setup_menubar(&mut siv, handle.clone());
    siv.set_autohide_menu(false);

    // blocks view
   // let mut block_storage = BlockStorage::new();
   // blocks::start_block_refresh(&mut siv, &mut block_storage);

    // "q" to quit
    siv.add_global_callback('q', |s| exit::show_exit_dialog(s));

    siv.run();

    Ok(())
}

