mod cli;
mod ui;

use std::error::Error;
use cursive::{Cursive, CursiveExt};
use ui::exit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

let mut siv = Cursive::default();

    ui::blocks::blocks_view(&mut siv);

    // "b" key to blocks
    siv.add_global_callback('b', |s| {
        s.pop_layer(); 
        ui::blocks::blocks_view(s);
    });

    // "q" to quit
    siv.add_global_callback('q', |s| exit::show_exit_dialog(s));

    siv.run();

    Ok(())
}

