mod api;
mod app_core;
mod data;
mod ui;

use cursive::{Cursive, CursiveExt};
use cursive::views::{TextView, LinearLayout, Dialog};
use std::error::Error;
use cursive::view::Nameable;
use std::time::Duration;
use tokio::time;
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

    // Add a counter TextView at the top of the layout
    let counter_view = TextView::new("Updating in 5 seconds");
    let counter_view_name = "counter_view";
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(counter_view.with_name(counter_view_name)) // Add the counter
        ).title("Onchain")
    );

    // Blocks view (initial load)
    blocks_view(&mut siv);

    // "q" to quit
    siv.add_global_callback('q', |s| show_exit_dialog(s));

    // Spawn a separate task to update the blocks view every 5 seconds and update the counter every second
    let cb_sink = siv.cb_sink().clone();
    tokio::spawn(async move {
        let mut countdown = 5;
        let mut interval = time::interval(Duration::from_secs(1)); // 1-second interval
        loop {
            interval.tick().await;
            countdown -= 1;

            // Update the counter display
            let _ = cb_sink.send(Box::new(move |s| {
                let counter_text = format!("Updating in {} seconds", countdown);
                if let Some(mut view) = s.find_name::<TextView>(counter_view_name) {
                    view.set_content(counter_text);
                }
            }));

            // If countdown reaches 0, reset to 5 and update the blocks view
            if countdown == 0 {
                countdown = 5;
                let _ = cb_sink.send(Box::new(|s| {
                    blocks_view(s);
                }));
            }
        }
    });

    siv.run();

    Ok(())
}
