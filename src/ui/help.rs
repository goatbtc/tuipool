use crate::app_core::*;
use cursive::Cursive;
use feerates::get_recent_feerates;
use mempool::check_transaction;

pub async fn local_process_command(siv: &mut Cursive, command: &str) -> String {
    let args: Vec<&str> = command.split_whitespace().collect();
    match args[0] {
        "feerates" | "recent_feerates" => get_recent_feerates().await.unwrap(),
        "check_tx" | "check_txid" => {
            if args.len() > 1 {
                check_transaction(args[1]).await.unwrap()
            } else {
                "Error: No txid provided. Usage: check_tx <txid>".to_string()
            }
        }
        "estimate_time" | "estimate_confirm" => {
            if args.len() > 1 {
                "Error: No txid provided. Usage: estimate_time <txid>".to_string()
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
