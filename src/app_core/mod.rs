pub mod estimate;
pub mod feerates;
pub mod mempool;

use cursive::Cursive;
use tokio::runtime::Runtime;

// Function to process each CLI command
pub fn local_process_command(_siv: &mut Cursive, command: &str) -> String {
    // Split the command input to handle arguments
    let args: Vec<&str> = command.split_whitespace().collect();
    if args.is_empty() {
        return "No command provided. Type 'help' for a list of commands.".to_string();
    }

    // Create a Tokio runtime to run async functions in a blocking context
    let rt = Runtime::new().expect("Failed to create runtime");

    match args[0] {
        "feerates" => {
            // Call the feerates function and get the output
            rt.block_on(feerates::get_recent_feerates())
                .unwrap_or_else(|_| "Failed to fetch fees.".to_string())
        }
        "check" => {
            if args.len() > 1 {
                // Check the transaction ID status
                rt.block_on(mempool::check_transaction(args[1]))
                    .unwrap_or_else(|_| "Failed to check transaction.".to_string())
            } else {
                "Error: No txid provided. Usage: check <txid>".to_string()
            }
        }
        "eta" => {
            if args.len() > 1 {
                // Parse the fee rate and optionally transaction size if provided
                let fee_rate: f64 = args[1].parse().unwrap_or(0.0);
                let tx_size = if args.len() > 2 {
                    args[2].parse().ok()
                } else {
                    None
                };
                estimate::estimate_confirmation_time(fee_rate, tx_size)
            } else {
                "Error: No fee rate provided. Usage: eta <fee_rate> [tx_size]".to_string()
            }
        }
        "help" => get_help_message(),
        _ => "Unknown command. Type 'help' for a list of commands.".to_string(),
    }
}

// Help message for the CLI
pub fn get_help_message() -> String {
    "Available commands:\n\
    - feerates: Show recent fee rates for different confirmation speeds\n\
    - check <txid>: Check if a given transaction ID is in the mempool\n\
    - eta <fee_rate> [tx_size]: Estimate confirmation time and optionally calculate estimated cost if tx_size is provided\n\
    - help: Show this list of commands".to_string()
}
