use cursive::views::{Dialog, TextView};
use cursive::Cursive;

pub fn blocks_view(siv: &mut Cursive) {
    let mempool_view = Dialog::around(TextView::new("Mempool Transactions"))
        .title("Blocks mempool view");

    siv.add_layer(mempool_view);
}
