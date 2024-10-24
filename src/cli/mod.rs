use std::error::Error;
use reqwest;
use serde::Deserialize;

const MEMPOOL_URL: &str = "https://mempool.space/api/mempool";  // Mantém a URL original para o mempool
const BLOCKSTREAM_FEE_URL: &str = "https://blockstream.info/api/fee-estimates";  // URL de taxas da Blockstream

#[derive(Deserialize, Debug)]
struct MempoolData {
    count: u64,
    vsize: u64,
}

#[derive(Deserialize, Debug)]
struct FeeEstimates {
    #[serde(rename = "1")]
    fastest_fee: f64,  // Taxa para confirmação em 1 bloco
    #[serde(rename = "3")]
    three_blocks_fee: f64,  // Taxa para confirmação em 3 blocos
    #[serde(rename = "6")]
    six_blocks_fee: f64,  // Taxa para confirmação em 6 blocos
}

pub async fn run_cli(fee_min: Option<f64>, size_min: Option<u64>) -> Result<(), Box<dyn Error>> {
    println!("Consultando dados do mempool...");

    // Faz a requisição para a API pública do mempool
    let mempool_response = reqwest::get(MEMPOOL_URL).await?;
    let mempool_data: MempoolData = mempool_response.json().await?;

    // Exibe informações básicas sobre o mempool
    println!("Total de transações no mempool: {}", mempool_data.count);
    println!("Tamanho total do mempool (em vbytes): {}", mempool_data.vsize);

    // Faz a requisição para a API pública da Blockstream para taxas estimadas
    println!("Consultando taxas de transação estimadas...");
    let fee_response = reqwest::get(BLOCKSTREAM_FEE_URL).await?;
    let fee_data: FeeEstimates = fee_response.json().await?;

    // Exibe as taxas estimadas
    println!("Taxa mais rápida (1 bloco): {:.2} sat/vByte", fee_data.fastest_fee);
    println!("Taxa para 3 blocos: {:.2} sat/vByte", fee_data.three_blocks_fee);
    println!("Taxa para 6 blocos: {:.2} sat/vByte", fee_data.six_blocks_fee);

    // Implementação de filtros (se necessário, para futuras funcionalidades)
    if let Some(fee_min) = fee_min {
        println!("Filtro de taxa mínima aplicada: {} sat/vByte", fee_min);
        // Implementar lógica para lidar com o filtro de taxa mínima, se necessário
    }

    if let Some(size_min) = size_min {
        println!("Filtro de tamanho mínimo aplicado: {} bytes", size_min);
        // Implementar lógica para lidar com o filtro de tamanho mínimo, se necessário
    }

    Ok(())
}

