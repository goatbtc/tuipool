use cursive::menu::Tree as MenuTree;
use cursive::Cursive;
use cursive::event::Key;

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
                })
        )
        .add_leaf("Quit", |s| crate::ui::exit::show_exit_dialog(s) ) ;

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}
