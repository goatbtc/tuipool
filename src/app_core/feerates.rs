pub fn get_recent_feerates() -> String {
    // Simulated fee rate values; in a real scenario, you'd fetch this data from an API
    let fast = 15.0;
    let medium = 10.0;
    let slow = 5.0;
    let expulsion_threshold = 2.0;

    format!(
        "Fetching recent fee rates...\n\
        - Fast Confirmation: {:.2} sat/vB\n\
        - Medium Confirmation: {:.2} sat/vB\n\
        - Slow Confirmation: {:.2} sat/vB\n\
        - Expulsion Threshold: {:.2} sat/vB",
        fast, medium, slow, expulsion_threshold
    )
}

