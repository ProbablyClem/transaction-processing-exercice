use super::transaction::Transaction;

#[derive(Debug)]
pub enum Message {
    Transaction(Transaction),
    Eof,
}
