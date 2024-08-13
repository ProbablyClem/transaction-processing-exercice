use super::transaction_type::TransactionType;

#[derive(Debug, serde::Deserialize)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}
