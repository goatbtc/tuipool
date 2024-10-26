use cursive::views::{Dialog, TextView};
use cursive::Cursive;

pub fn show_exit_dialog(siv: &mut Cursive) {
    let exit_dialog = Dialog::new()
        .title("Wanna exit?")
        .content(TextView::new("Are you sure you want to exit?"))
        .button("YES", |s| s.quit()) // Saindo do aplicativo
        .button("CANCEL", |s| {
            s.pop_layer(); // Fecha o diálogo de saída
        });

    siv.add_layer(exit_dialog);
}

