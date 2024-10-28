mod api;
mod app_core;
mod data;
mod ui;

use cursive::{Cursive, CursiveExt};
use std::error::Error;
use tokio::time;
use ui::menubar::setup_menubar;
use ui::blocks::blocks_view;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Our TUI
    let mut siv = Cursive::default();

    // Menubar setup
    setup_menubar(&mut siv);
    siv.set_autohide_menu(false);

    // // Blocks view (initial load)
    // blocks_view(&mut siv);
    //
    // "q" to quit
    siv.add_global_callback('q', |s| s.quit());

    // Start periodic block updates
    let cb_sink = siv.cb_sink().clone();
    tokio::spawn(async move {
        loop {
            // Wait for 5 seconds
            time::sleep(time::Duration::from_secs(5)).await;

            // Call blocks_view to update the UI with the latest blocks
            let cb_sink = cb_sink.clone();
            let _ = cb_sink.send(Box::new(move |s| {
                ui::blocks::blocks_view(s);
            }));
        }
    });

    // Run the Cursive event loop
    siv.run();

    Ok(())
}

