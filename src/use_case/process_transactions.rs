use crate::model::{account::Account, transaction::Transaction};

pub fn process_transactions(transactions: Vec<Transaction>) -> Vec<Account> {
    let mut accounts = std::collections::HashMap::new();
    for t in transactions {
        t.execute(&mut accounts);
    }
    accounts
        .iter()
        .map(|(id, account)| account.to_owned())
        .collect()
}
