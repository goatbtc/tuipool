use cursive::event::Key;
use cursive::menu::{self, Tree as MenuTree};
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

use crate::api::client::*;

pub fn setup_menubar(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree(
            "Mempool",
            MenuTree::new()
                .leaf("Mempool data", |s| {
                    show_mempool_data(s);
                })
                .leaf("Average Fees", |s| {
                    setup_mempool_menu(s);
                }),
        )
        .add_subtree(
            "Help",
            menu::Tree::new().leaf("About", |s| {
                s.add_layer(Dialog::info("tuipool - mempool visualizer"));
            }),
        )
        .add_leaf("Quit", |s| crate::ui::exit::show_exit_dialog(s));

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_global_callback('c', |s| {
        s.add_layer(
            Dialog::new()
                .title("Command Line Interface")
                .content(EditView::new().on_submit(|s, input| {}))
                .button("Cancel", |s| {
                    s.pop_layer();
                }),
        );
    });
}
