use crate::model::message::Message;
use crate::model::transaction::Transaction;
use tokio::fs::File;
use tokio::io::AsyncRead;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

pub async fn read_transactions(file_name: String, sender: mpsc::Sender<Message>) {
    let reader = File::open(file_name).await.expect("file not found");
    read_csv(reader, sender).await;
}

async fn read_csv<T: AsyncRead + Send + Unpin>(reader: T, sender: mpsc::Sender<Message>) {
    let mut reader = csv_async::AsyncReaderBuilder::new()
        .trim(csv_async::Trim::All)
        .create_deserializer(reader);

    //Deserialize the CSV asynchronously and send each transaction to the receiver.
    while let Some(transaction) = reader.deserialize::<Transaction>().next().await {
        let transaction = transaction.expect("Error reading transaction");
        sender
            .send(Message::Transaction(transaction))
            .await
            .unwrap()
    }

    // Once we reach the end of the file, we send an Eof message to the receiver, to signal that we are done.
    sender.send(Message::Eof).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Collect transactions from the receiver and return them as a Vec.
    // This function will be used in the tests to collect transactions from the receiver.
    async fn collect(mut receiver: mpsc::Receiver<Message>) -> Vec<Transaction> {
        let mut transactions = Vec::new();
        while let Some(transaction) = receiver.recv().await {
            match transaction {
                Message::Transaction(txn) => {
                    transactions.push(txn);
                }
                Message::Eof => {
                    //Once we reach the end of the file, we break the loop
                    break;
                }
            }
        }
        transactions
    }

    async fn parse_data_sync(data: &str) -> Vec<Transaction> {
        let (sender, receiver) = mpsc::channel(100);
        read_csv(data.as_bytes(), sender).await;
        collect(receiver).await
    }

    #[tokio::test]
    async fn test_read_csv_file() {
        // Test reading from an actual CSV file
        let (sender, receiver) = mpsc::channel(100);
        read_transactions("test/transactions.csv".to_string(), sender).await;
        let transactions = collect(receiver).await;
        assert_eq!(transactions.len(), 5);
    }

    #[tokio::test]
    async fn test_read_csv() {
        let data = "\
            type, client, tx, amount
            deposit, 1, 1, 1.0
            deposit, 2, 2, 2.0
            deposit, 1, 3, 2.0
            withdrawal, 1, 4, 1.5
            withdrawal, 2, 5, 3.0";

        let transactions = parse_data_sync(data).await;
        assert_eq!(transactions.len(), 5);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_read_csv_with_invalid_type() {
        let data = "\
            type, client, tx, amount
            invalid, 2, 6, 3.0
            deposit, 1, 7, 1.0";
        // invalid type

        parse_data_sync(data).await;
    }

    #[tokio::test]
    async fn test_read_csv_with_optional_amount() {
        let data = "\
            type, client, tx, amount
            deposit, 1, 1, 1.0
            deposit, 2, 2, 2.0
            deposit, 1, 3, 2.0
            withdrawal, 1, 4, 1.5
            withdrawal, 2, 5, ";
        // invalid amount

        let transactions = parse_data_sync(data).await;
        assert_eq!(transactions.len(), 5);
    }

    #[tokio::test]
    async fn test_read_csv_with_precise_amount() {
        let data = "\
            type, client, tx, amount
            withdrawal, 2, 5, 3.0001";

        let transactions = parse_data_sync(data).await;
        let transaction = &transactions[0];
        assert_eq!(transaction.amount(), 3.0001);
    }

    #[tokio::test]
    async fn test_read_csv_with_spaces() {
        let data = "\
            type, client, tx, amount
            withdrawal,     2, 5    , 3.0";
        // random spaces

        let transactions = parse_data_sync(data).await;
        assert_eq!(transactions.len(), 1);
    }
}
