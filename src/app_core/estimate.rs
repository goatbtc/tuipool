pub fn estimate_confirmation_time(fee_rate: f64) -> String {
    if fee_rate >= 15.0 {
        "Estimated confirmation in 1-2 blocks.".to_string()
    } else if fee_rate >= 10.0 {
        "Estimated confirmation in 3-6 blocks.".to_string()
    } else {
        "Estimated confirmation in over 6 blocks.".to_string()
    }
}

