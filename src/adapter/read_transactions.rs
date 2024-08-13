use csv::Reader;

use crate::model::transaction::Transaction;

pub fn read_transactions(file_name: String) -> Vec<Transaction> {
    let reader = std::fs::File::open(file_name).expect("file not found");
    read_csv(reader).expect("Error parsing csv file")
}

fn read_csv<T: std::io::Read>(reader: T) -> Result<Vec<Transaction>, anyhow::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader);

    let mut transactions = Vec::new();
    for result in reader.records() {
        let record = result?;
        let record: Transaction = record.deserialize(None)?;
        transactions.push(record)
    }
    Ok(transactions)
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
        assert_eq!(transactions.unwrap().len(), 5);
    }

    #[test]
    fn test_read_csv_with_invalid_type() {
        let data = "\
            type, client, tx, amount
            invalid, 2, 6, 3.0
            deposit, 1, 7, 1.0";
        // invalid type
        let transactions = read_csv(data.as_bytes());
        assert!(transactions.is_err());
    }

    #[test]
    fn test_read_csv_with_optional_amount() {
        let data = "\
            type, client, tx, amount
            deposit, 1, 1, 1.0
            deposit, 2, 2, 2.0
            deposit, 1, 3, 2.0
            withdrawal, 1, 4, 1.5
            withdrawal, 2, 5, ";
        // invalid amount
        let transactions = read_csv(data.as_bytes()).unwrap();
        assert_eq!(transactions.len(), 5);
    }

    #[test]
    fn test_read_csv_with_precise_amount() {
        let data = "\
            type, client, tx, amount
            withdrawal, 2, 5, 3.001";

        let transactions = read_csv(data.as_bytes()).unwrap();
        let transaction = &transactions[0];
        assert_eq!(transaction.amount, Some(3.001));
    }

    #[test]
    fn test_read_csv_with_spaces() {
        let data = "\
            type, client, tx, amount
            withdrawal,     2, 5    , 3.0";
        // random spaces
        let transactions = read_csv(data.as_bytes()).unwrap();
        assert_eq!(transactions.len(), 1);
    }
}
