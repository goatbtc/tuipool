use cursive::{menu, Cursive, views::Dialog, event::Key};

pub fn setup_menubar(siv: &mut Cursive){
    siv.menubar()
        .add_subtree(
            "Blocks",
            menu::Tree::new()
                .leaf("Blocks", |s| {
                    s.add_layer(Dialog::info("About this application"));
                }),
        )
        .add_subtree(
            "Help",
            menu::Tree::new()
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("About this application"));
                }),
        )
        .add_leaf("Quit", |s| crate::exit::show_exit_dialog(s)) ;

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}

