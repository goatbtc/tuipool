use cursive::views::{Dialog, TextView};
use cursive::Cursive;

pub fn show_exit_dialog(siv: &mut Cursive) {
    let exit_dialog = Dialog::new()
        .title("Wanna exit?")
        .content(TextView::new("Are you sure you want to exit?"))
        .button("YES", |s| s.quit())
        .button("CANCEL", |s| {
            s.pop_layer();
        });

    siv.add_layer(exit_dialog);
}
