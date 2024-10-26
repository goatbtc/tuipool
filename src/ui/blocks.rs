use cursive::views::{Dialog, LinearLayout, Panel, TextView};
use cursive::Cursive;
use rand::Rng;
// src/ui/blocks.rs
use crate::BlockStorage;
use crate::data::data::BlockData;

pub fn create_block_view(block: BlockData) -> Panel<TextView> {
    let block_info = format!(
        "Height: {}\n{:.2} sat/vB\n{} transactions\n{:.8} BTC\n{}",
        block.height, block.sat_per_vbyte, block.transactions, block.btc_amount, block.time
    );
    

    Panel::new(TextView::new(block_info)).title(block.pool)
}

fn render_blocks(siv: &mut Cursive, blocks: &[BlockData]) {
    let mut layout = LinearLayout::horizontal();

    for block in blocks {
        layout.add_child(create_block_view(block.clone()));
    }

    siv.add_layer(Dialog::around(layout).title("Onchain"));

}

pub fn start_block_refresh(siv: &mut Cursive, block_storage: &mut BlockStorage) {
    let new_block_data = get_new_block_data();

    for block in new_block_data {
        block_storage.add_block(block);
    }

    render_blocks(siv, block_storage.get_blocks());
}

fn get_new_block_data() -> Vec<BlockData> {
    let mut rng = rand::thread_rng();

    // Simulação de 10 blocos com valores fictícios
    (0..4)
        .map(|i| BlockData {
            height: 867290 + i as u64, // Incrementa a altura do bloco
            sat_per_vbyte: rng.gen_range(1.0..10.0), // Taxa aleatória
            transactions: rng.gen_range(1000..5000), // Número de transações
            btc_amount: rng.gen_range(0.1..0.5), // Quantidade de BTC
            time: format!("{} minutes ago", rng.gen_range(1..60)), // Tempo simulado
            pool: format!("Cluster{}", i + 1), // Nome fictício do pool minerador
        })
        .collect()
}

