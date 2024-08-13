use super::transaction_type::TransactionType;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

impl Transaction {
    pub fn amount(&self) -> f64 {
        //Sanity check
        match self.transaction_type {
            TransactionType::Deposit => self.amount.unwrap(),
            TransactionType::Withdrawal => self.amount.unwrap(),
            _ => panic!(
                "{:?} Transaction does not have an amount",
                self.transaction_type
            ),
        }
    }
}
