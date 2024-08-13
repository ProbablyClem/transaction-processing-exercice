use csv::Reader;

use crate::model::transaction::Transaction;

pub fn read_transactions(file_name: String) -> Vec<Transaction> {
    let reader = std::fs::File::open(file_name).unwrap();
    read_csv(reader)
}

fn read_csv<T: std::io::Read>(reader: T) -> Vec<Transaction> {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader);

    let mut transactions = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        let record: Transaction = record.deserialize(None).unwrap();
        transactions.push(record)
    }
    transactions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv_file() {
        let transactions = read_transactions("test/transactions.csv".to_string());
        assert_eq!(transactions.len(), 5);
    }

    #[test]
    fn test_read_csv() {
        let data = "\
            type, client, tx, amount
            deposit, 1, 1, 1.0
            deposit, 2, 2, 2.0
            deposit, 1, 3, 2.0
            withdrawal, 1, 4, 1.5
            withdrawal, 2, 5, 3.0";

        let transactions = read_csv(data.as_bytes());
        assert_eq!(transactions.len(), 5);
    }

    #[test]
    fn test_read_csv_with_invalid_data() {
        let data = "\
            type, client, tx, amount
            invalid, 2, 6, 3.0
            deposit, 1, 7, 1.0";

        let transactions = read_csv(data.as_bytes());
        assert_eq!(transactions.len(), 1);
    }
}
