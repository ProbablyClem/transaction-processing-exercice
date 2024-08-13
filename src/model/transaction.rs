use super::{account::Account, transaction_type::TransactionType};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

impl Transaction {
    pub fn execute(&self, accounts: &mut std::collections::HashMap<u16, Account>) {
        let account = accounts
            .entry(self.client)
            .or_insert(Account::new(self.client));
        account.transactions.insert(self.tx, self.clone());
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
