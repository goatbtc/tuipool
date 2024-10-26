pub mod mempool;
pub mod estimate;
pub mod feerates;

use cursive::Cursive;

// Simplified function to process each CLI command without async for stability
pub fn local_process_command(_siv: &mut Cursive, command: &str) -> String {
    // Split the command input to handle arguments
    let args: Vec<&str> = command.split_whitespace().collect();
    if args.is_empty() {
        return "No command provided. Type 'help' for a list of commands.".to_string();
    }

    match args[0] {
        "feerates" => "Sample output for feerates command.".to_string(),
        "check" => {
            if args.len() > 1 {
                format!("Sample output for checking txid: {}", args[1])
            } else {
                "Error: No txid provided. Usage: check <txid>".to_string()
            }
        }
        "eta" => {
            if args.len() > 1 {
                format!("Sample output for ETA with fee rate: {}", args[1])
            } else {
                "Error: No fee rate provided. Usage: eta <fee_rate>".to_string()
            }
        }
        "help" => get_help_message(),
        _ => "Unknown command. Type 'help' for a list of commands.".to_string(),
    }
}

// Help message for the CLI
fn get_help_message() -> String {
    "Available commands:\n\
    - feerates: Show recent fee rates for different confirmation speeds\n\
    - check <txid>: Check if a given transaction ID is in the mempool\n\
    - eta <fee_rate>: Estimate confirmation time based on fee rate\n\
    - help: Show this list of commands".to_string()
}

