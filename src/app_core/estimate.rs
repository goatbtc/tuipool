/// Estimates the confirmation time based on the fee rate and optionally calculates the estimated cost
pub fn estimate_confirmation_time(fee_rate: f64, tx_size: Option<u64>) -> String {
    let confirmation = if fee_rate >= 15.0 {
        "Estimated confirmation in 1-2 blocks"
    } else if fee_rate >= 10.0 {
        "Estimated confirmation in 3-6 blocks"
    } else {
        "Estimated confirmation in over 6 blocks"
    };

    if let Some(size) = tx_size {
        let estimated_cost = fee_rate * size as f64;
        format!(
            "{}. Estimated cost: {:.2} satoshis.",
            confirmation, estimated_cost
        )
    } else {
        format!("{}.", confirmation)
    }
}
