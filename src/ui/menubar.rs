use cursive::menu::Tree as MenuTree;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

use crate::api::client::show_mempool_data;

pub fn setup_menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree(
            "Mempool",
            MenuTree::new()
                .leaf("Average Fees", |s| {
                    s.add_layer(Dialog::info("Displaying average fees..."));
                })
                .subtree(
                    "Tx-Specific",
                    MenuTree::new()
                        .leaf("TxID", |s| {
                            s.add_layer(
                                Dialog::new()
                                    .title("Enter Transaction ID")
                                    .content(
                                        EditView::new()
                                            .on_submit(|s, txid| {
                                                let response =
                                                    format!("Transaction ID entered: {}", txid);
                                                s.pop_layer();
                                                s.add_layer(Dialog::info(response));
                                            })
                                            .with_name("txid_input")
                                            .fixed_width(40),
                                    )
                                    .button("Cancel", |s| {
                                        s.pop_layer();
                                    }),
                            );
                        })
                        .leaf("Serialized", |s| {
                            s.add_layer(
                                Dialog::new()
                                    .title("Enter Serialized Transaction")
                                    .content(
                                        EditView::new()
                                            .on_submit(|s, hex| {
                                                let response = format!(
                                                    "Serialized transaction entered: {}",
                                                    hex
                                                );
                                                s.pop_layer();
                                                s.add_layer(Dialog::info(response));
                                            })
                                            .with_name("hex_input")
                                            .fixed_width(40),
                                    )
                                    .button("Cancel", |s| {
                                        s.pop_layer();
                                    }),
                            );
                        }),
                ),
        )
        .add_leaf("Teste", |s| {
            show_mempool_data(s);
        } ) 
        .add_leaf("Quit", |s| crate::ui::exit::show_exit_dialog(s) ) ;

}
