use adapter::{read_transactions::read_transactions, write_accounts::write_accounts};
use use_case::process_transactions::process_transactions;

mod adapter;
mod model;
mod use_case;
fn main() {
    let file_name = get_file_name();
    let transactions = read_transactions(file_name);
    let accounts = process_transactions(transactions);
    write_accounts(accounts)
}

fn get_file_name() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    let file_name = &args[1];
    file_name.to_string()
}
