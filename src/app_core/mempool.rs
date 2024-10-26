pub fn check_transaction(txid: &str) -> String {
    // Conectar com API para verificar se o txid está no mempool
    // Simulação para agora
    let found = true; // Aqui faremos uma verificação real
    if found {
        format!("Transaction {} found in the mempool.", txid)
    } else {
        format!("Transaction {} not found.", txid)
    }
}

pub fn check_txid_in_mempool(txid: &str) -> String {
    format!("Checking if transaction {} is in the mempool...", txid)
}

