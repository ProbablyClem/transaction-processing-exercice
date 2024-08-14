use crate::model::account::Account;

pub async fn write_accounts(accounts: Vec<Account>) {
    let accounts = parse_accounts(accounts)
        .await
        .expect("Failed to parse accounts");
    print!("{}", accounts);
}

async fn parse_accounts(accounts: Vec<Account>) -> Result<String, anyhow::Error> {
    let mut wtr = csv_async::AsyncSerializer::from_writer(vec![]);
    for account in accounts {
        wtr.serialize(account).await?;
    }
    Ok(String::from_utf8(wtr.into_inner().await?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::account::Account;

    #[tokio::test]
    async fn test_write_accounts() {
        let accounts = vec![
            Account {
                client: 1,
                available: 1.0,
                ..Default::default()
            },
            Account {
                client: 2,
                available: 2.0,
                locked: true,
                ..Default::default()
            },
        ];
        let result = parse_accounts(accounts).await;
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
