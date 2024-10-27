use cursive::{menu, Cursive, views::{Dialog, EditView}, event::{Key}};
use crate::app_core::{local_process_command, feerates, mempool};
use crate::onchain;


pub fn setup_menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_leaf("Onchain", |s| {
            onchain::show_onchain_data(s);
        })
        .add_subtree(
            "Mempool",
            menu::Tree::new()
                .leaf("Average Fees", |s| {
                    feerates::show_average_fees(s);
                })
                .leaf("TX-Specific", |s| {
                    mempool::show_transaction_check(s);
                }),
        )
        .add_leaf("CLI", |s| {
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
        })
        .add_leaf("Help", |s| {
            s.add_layer(Dialog::info(crate::app_core::get_help_message()));
        })
        .add_leaf("Quit", |s| crate::exit::show_exit_dialog(s));

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    
    // Set up 'c' key to directly open the CLI interface
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

