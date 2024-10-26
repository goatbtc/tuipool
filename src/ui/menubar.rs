use cursive::{menu, Cursive, views::{Dialog, EditView}, event::Key};
use crate::{app_core::local_process_command, onchain::show_onchain_data, mempool::show_mempool_data};
use futures::executor::block_on; // Import for blocking async functions

pub fn setup_menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree(
            "Onchain",
            menu::Tree::new()
                .leaf("View Onchain Data", |s| {
                    // Block on the async function instead of spawning
                    block_on(show_onchain_data(s));
                }),
        )
        .add_subtree(
            "Mempool",
            menu::Tree::new()
                .leaf("View Mempool Data", |s| {
                    // Block on the async function instead of spawning
                    block_on(show_mempool_data(s));
                }),
        )
        .add_subtree(
            "Help",
            menu::Tree::new()
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("This application provides real-time Bitcoin on-chain and mempool data."));
                }),
        )
        .add_leaf("Quit", |s| crate::exit::show_exit_dialog(s));

    // Set up shortcut for Esc key to open the menu
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    // CLI that opens on pressing 'c'
    siv.add_global_callback('c', |s| {
        s.add_layer(
            Dialog::new()
                .title("Command Line Interface")
                .content(EditView::new().on_submit(|s, input| {
                    let response = local_process_command(s, input);
                    s.pop_layer();
                    s.add_layer(Dialog::info(response));
                }))
                .button("Cancel", |s| {
                    s.pop_layer();
                }),
        );
    });
}

