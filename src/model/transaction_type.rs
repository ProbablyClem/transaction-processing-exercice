#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trasaction_type_deserialize() {
        assert!(serde_json::from_str::<TransactionType>("\"deposit\"").is_ok());
    }
}
