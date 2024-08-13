use crate::model::account::Account;

pub fn write_accounts(accounts: Vec<Account>) {
    let accounts = parse_accounts(accounts).expect("Failed to parse accounts");
    print!("{}", accounts);
}

fn parse_accounts(accounts: Vec<Account>) -> Result<String, anyhow::Error> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for account in accounts {
        wtr.serialize(account)?;
    }
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::account::Account;

    #[test]
    fn test_write_accounts() {
        let accounts = vec![
            Account {
                client: 1,
                available: 1.0,
                held: 0.0,
                locked: false,
            },
            Account {
                client: 2,
                available: 2.0,
                held: 0.0,
                locked: true,
            },
        ];
        let result = parse_accounts(accounts);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            "client,available,held,total,locked
1,1.0,0.0,1.0,false
2,2.0,0.0,2.0,true
"
        );
    }
}
