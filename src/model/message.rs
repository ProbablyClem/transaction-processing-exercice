use super::transaction::Transaction;

#[derive(Debug)]
/// Message enum to represent the different types of messages that can be sent through the channel
pub enum Message {
    Transaction(Transaction),
    Eof,
}
