use super::{account::Account, transaction_type::TransactionType};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    amount: Option<f64>,
}

impl Transaction {
    pub fn amount(&self) -> f64 {
        match self.transaction_type {
            TransactionType::Deposit => self.amount.unwrap(),
            TransactionType::Withdrawal => self.amount.unwrap(),
            _ => panic!(
                "{:?} Transaction does not have an amount",
                self.transaction_type
            ),
        }
    }

    pub fn execute(&self, accounts: &mut std::collections::HashMap<u16, Account>) {
        let account = accounts
            .entry(self.client)
            .or_insert(Account::new(self.client));

        if account.locked {
            // If the account is locked, store the transaction in a separate map, to not lose the data
            account.locked_transactions.insert(self.tx, self.clone());
            return;
        }

        account.transactions.insert(self.tx, self.clone());

        match self.transaction_type {
            TransactionType::Deposit => {
                account.available += self.amount();
            }
            TransactionType::Withdrawal => {
                account.available -= self.amount();
            }
            TransactionType::Dispute => {
                // If the transaction is not found, do nothing
                if let Some(transaction) = account.transactions.get(&self.tx) {
                    account.available -= transaction.amount();
                    account.held += transaction.amount();
                }
            }
            TransactionType::Resolve => {
                // If the transaction is not found, do nothing
                if let Some(transaction) = account.transactions.get(&self.tx) {
                    account.available += transaction.amount();
                    account.held -= transaction.amount();
                }
            }
            TransactionType::Chargeback => {
                // If the transaction is not found, do nothing
                let transaction = account.transactions.get(&self.tx).unwrap();
                account.held -= transaction.amount();
                account.locked = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let mut accounts = std::collections::HashMap::new();
        let transaction = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(1.0),
        };
        transaction.execute(&mut accounts);
        assert!(accounts.contains_key(&1));
    }
}
