use super::transaction_type::TransactionType;

// I usually like to make this an enum to enforce correctness with the type system
// I would use the TransactionType as the variant and put the fields in the correct cases
// This way, we could remove the option arount the amount and only specify it in the correct cases
// Unforunately, the csv parser crate does not support tagged union https://github.com/BurntSushi/rust-csv/issues/211
#[derive(Debug, serde::Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
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
