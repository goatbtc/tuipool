pub mod mempool;
pub mod estimate;
pub mod feerates;

use cursive::Cursive;

// Function to process each CLI command and return the appropriate response
pub fn local_process_command(siv: &mut Cursive, command: &str) -> String {
    let args: Vec<&str> = command.split_whitespace().collect();
    match args[0] {
        "feerates" | "recent_feerates" => feerates::get_recent_feerates(),
        "check_tx" | "check_txid" => {
            if args.len() > 1 {
                mempool::check_transaction(args[1])
            } else {
                "Error: No txid provided. Usage: check_tx <txid>".to_string()
            }
        }
        "estimate_time" | "estimate_confirm" => {
            if args.len() > 1 {
                estimate::estimate_confirmation_time(args[1])
            } else {
                "Error: No txid provided. Usage: estimate_time <txid>".to_string()
            }
        }
        "help" => get_help_message(),
        _ => "Unknown command. Type 'help' for a list of commands.".to_string(),
    }
}

// Help message for the CLI
fn get_help_message() -> String {
    "Available commands:\n\
    - feerates or recent_feerates: Show recent fee rates for different confirmation speeds\n\
    - check_tx <txid> or check_txid <txid>: Check if a given transaction ID is in the mempool\n\
    - estimate_time <txid> or estimate_confirm <txid>: Estimate confirmation time for a given transaction ID\n\
    - help: Show this list of commands".to_string()
}

