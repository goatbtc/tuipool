use std::error::Error;
use reqwest;
use serde::Deserialize;

const MEMPOOL_URL: &str = "https://mempool.space/api/mempool";
const BLOCKSTREAM_FEE_URL: &str = "https://blockstream.info/api/fee-estimates";

#[derive(Deserialize, Debug)]
struct MempoolData {
    count: u64,
    vsize: u64,
}

#[derive(Deserialize, Debug)]
struct FeeEstimates {
    #[serde(rename = "1")]
    fastest_fee: f64,
    #[serde(rename = "3")]
    three_blocks_fee: f64,
    #[serde(rename = "6")]
    six_blocks_fee: f64,
}

pub async fn run_cli(_fee_min: Option<f64>, _size_min: Option<u64>) -> Result<(), Box<dyn Error>> {
    println!("Consultando dados do mempool...");

    let mempool_response = reqwest::get(MEMPOOL_URL).await?;
    let mempool_data: MempoolData = mempool_response.json().await?;

    println!("Total de transações no mempool: {}", mempool_data.count);
    println!("Tamanho total do mempool (em vbytes): {}", mempool_data.vsize);

    println!("Consultando taxas de transação estimadas...");
    let fee_response = reqwest::get(BLOCKSTREAM_FEE_URL).await?;
    let fee_data: FeeEstimates = fee_response.json().await?;

    println!("Taxa mais rápida (1 bloco): {:.2} sat/vByte", fee_data.fastest_fee);
    println!("Taxa para 3 blocos: {:.2} sat/vByte", fee_data.three_blocks_fee);
    println!("Taxa para 6 blocos: {:.2} sat/vByte", fee_data.six_blocks_fee);

    Ok(())
}

