use tokio::sync::mpsc;

use crate::model::{
    account::Account, message::Message, transaction::Transaction, transaction_type::TransactionType,
};

/// Process the transactions and return the accounts
pub async fn process_transactions(mut receiver: mpsc::Receiver<Message>) -> Vec<Account> {
    let mut accounts = std::collections::HashMap::new();

    // We process the transactions as they comme in
    while let Some(transaction) = receiver.recv().await {
        match transaction {
            Message::Transaction(txn) => {
                execute(txn, &mut accounts);
            }
            Message::Eof => {
                //Once we reach the end of the file, we break the loop
                break;
            }
        }
    }

    // We return the accounts
    accounts
        .values()
        .map(|account| account.to_owned())
        .collect()
}

/// Execute one transaction on the account
pub fn execute(txn: Transaction, accounts: &mut std::collections::HashMap<u16, Account>) {
    // Get the account or create a new one
    let account = accounts
        .entry(txn.client)
        .or_insert(Account::new(txn.client));

    if account.locked {
        // If the account is locked, store the transaction in a separate map, to not lose the data
        // In a real-world scenario, we should persist this somewhere
        account.locked_transactions.insert(txn.tx, txn);
        return;
    }

    match txn.transaction_type {
        TransactionType::Deposit => {
            account.available += txn.amount();
            account.transactions.insert(txn.tx, txn); // We insert after because insert takes ownership and we want to avoid cloning
        }
        TransactionType::Withdrawal => {
            account.available -= txn.amount();
            account.transactions.insert(txn.tx, txn); // We insert after because insert takes ownership and we want to avoid cloning
        }
        TransactionType::Dispute => {
            // If the transaction is not found, do nothing (defined in the spec)
            if let Some(source_txn) = account.transactions.get(&txn.tx) {
                account.available -= source_txn.amount();
                account.held += source_txn.amount();
            }
        }
        TransactionType::Resolve => {
            // If the transaction is not found, do nothing (defined in the spec)
            if let Some(source_txn) = account.transactions.get(&txn.tx) {
                account.available += source_txn.amount();
                account.held -= source_txn.amount();
            }
        }
        TransactionType::Chargeback => {
            // If the transaction is not found, do nothing (defined in the spec)
            if let Some(source_txn) = account.transactions.get(&txn.tx) {
                // Sanity check
                if account.held < source_txn.amount() {
                    panic!("Chargeback amount is greater than held amount, the transaction should be disputed first");
                }
                account.held -= source_txn.amount();
                account.locked = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::account;

    use super::*;

    #[tokio::test]
    async fn test_process_transactions() {
        let txn = vec![
            Transaction {
                transaction_type: crate::model::transaction_type::TransactionType::Deposit,
                client: 1,
                tx: 1,
                amount: Some(1.0),
            },
            Transaction {
                transaction_type: crate::model::transaction_type::TransactionType::Deposit,
                client: 2,
                tx: 2,
                amount: Some(2.0),
            },
        ];
        let (sender, receiver) = mpsc::channel(100);
        let accounts = process_transactions(receiver);
        for t in txn {
            sender.send(Message::Transaction(t)).await.unwrap();
        }
        sender.send(Message::Eof).await.unwrap();
        assert_eq!(accounts.await.len(), 2);
    }

    #[test]
    fn test_create_account() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };
        execute(transaction, &mut accounts);
        assert!(accounts.contains_key(&1));
    }

    #[test]
    fn test_deposit() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 1.0);
    }

    #[test]
    fn test_withdrawal() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, -1.0);
    }

    #[test]
    fn test_dispute() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };
        execute(transaction, &mut accounts);
        let transaction = Transaction {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 1,
            amount: None,
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 1.0);
    }

    #[test]
    fn test_resolve() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };
        execute(transaction, &mut accounts);
        let transaction = Transaction {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 1,
            amount: None,
        };

        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 1.0);
        let transaction = Transaction {
            transaction_type: TransactionType::Resolve,
            client: 1,
            tx: 1,
            amount: None,
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 1.0);
        assert_eq!(account.held, 0.0);
    }

    #[test]
    fn test_chargeback() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 1.0);
        assert_eq!(account.held, 0.0);
        let transaction = Transaction {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 1,
            amount: None,
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 1.0);
        let transaction = Transaction {
            transaction_type: TransactionType::Chargeback,
            client: 1,
            tx: 1,
            amount: None,
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 0.0);
        assert!(account.locked);
    }

    #[test]
    fn test_locked_account() {
        // Init the locked account
        let account = account::Account {
            client: 1,
            available: 0.0,
            held: 1.0,
            locked: true,
            ..Default::default()
        };

        let mut accounts = std::collections::HashMap::new();
        accounts.insert(1, account);

        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(1.0),
        };
        execute(transaction, &mut accounts);
        // The transaction should not be processed and the account should remain the same
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 1.0);
        assert!(account.locked);

        // The transaction should be stored in the locked_transactions map
        assert_eq!(account.locked_transactions.len(), 1);
    }

    #[test]
    fn test_precise_compute() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 1,
            amount: Some(3.0001),
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, -3.0001);
        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(3.0001),
        };
        execute(transaction, &mut accounts);
        let account = accounts.get(&1).unwrap();
        assert_eq!(account.available, 0.0);
    }
}
