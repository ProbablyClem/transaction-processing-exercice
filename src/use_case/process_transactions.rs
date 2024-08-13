use crate::model::{account::Account, transaction::Transaction};

pub fn process_transactions(transactions: Vec<Transaction>) -> Vec<Account> {
    let mut accounts = std::collections::HashMap::new();
    for t in transactions {
        accounts.insert(
            t.client,
            Account {
                client: t.client,
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: false,
            },
        );
    }
    accounts
        .iter()
        .map(|(id, account)| account.to_owned())
        .collect()
}
