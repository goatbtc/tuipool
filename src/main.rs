mod cli;
mod ui;
mod data;

use std::{error::Error, thread::sleep, time::Duration};
use cursive::{Cursive, CursiveExt};
use data::data::BlockStorage;
use ui::{blocks,exit,fees};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut siv = Cursive::default();

    let mut block_storage = BlockStorage::new();

    blocks::start_block_refresh(&mut siv, &mut block_storage);

    sleep(Duration::from_secs(2));

    blocks::start_block_refresh(&mut siv, &mut block_storage);

    // "q" to quit
    siv.add_global_callback('q', |s| exit::show_exit_dialog(s));

    siv.run();

    Ok(())
}

