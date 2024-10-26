pub fn estimate_confirmation_time(txid: &str) -> String {
    // Utilizar um modelo de ML para estimar o tempo de confirmação
    // Por enquanto, simulação de tempo
    let estimated_time = 15; // Exemplo de retorno do modelo
    format!("Estimated confirmation time for {}: {} minutes.", txid, estimated_time)
}

