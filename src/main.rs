use adapter::{read_transactions::read_transactions, write_accounts::write_accounts};
use tokio::sync::mpsc;
use use_case::process_transactions::process_transactions;

mod adapter;
mod model;
mod use_case;

#[tokio::main]
async fn main() {
    let file_name = get_file_name();

    // Create a channel with a buffer size of 100
    let (sender, receiver) = mpsc::channel(100);

    // We read the transactions from the file asynchronously and send them to the receiver channel
    read_transactions(file_name, sender).await;
    // We process the transactions as they comme in, and return the accounts once all transactions have been processed
    let accounts = process_transactions(receiver).await;

    write_accounts(accounts).await
}

/// Get file name from command line arguments
fn get_file_name() -> String {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <file_name>");
        std::process::exit(1);
    }

    let file_name = &args[1];
    file_name.to_string()
}
