use cursive::{menu, Cursive, views::{Dialog, EditView}, event::{Key}};


// Função para processar comandos diretamente
fn local_process_command(_s: &mut Cursive, input: &str) -> String {
    match input {
        "show fees" => "Displaying fee rates...".to_string(),
        "check txid" => "Checking if TXID is in mempool...".to_string(),
        "estimate time" => "Estimating confirmation time...".to_string(),
        _ => "Command not recognized.".to_string(),
    }
}

pub fn setup_menubar(siv: &mut Cursive) {
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
        .add_leaf("Quit", |s| crate::exit::show_exit_dialog(s));

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    // CLI que abre ao pressionar 'c'
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

