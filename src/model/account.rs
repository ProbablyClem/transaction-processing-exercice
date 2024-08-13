use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Account {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub locked: bool,
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = serializer.serialize_struct("Account", 5)?;
        s.serialize_field("client", &self.client)?;
        s.serialize_field("available", &self.available)?;
        s.serialize_field("held", &self.held)?;
        s.serialize_field("total", &self.total())?;
        s.serialize_field("locked", &self.locked)?;
        s.end()
    }
}

impl Account {
    pub fn new(client: u16) -> Self {
        Account {
            client,
            available: 0.0,
            held: 0.0,
            locked: false,
        }
    }

    pub fn total(&self) -> f64 {
        self.available + self.held
    }
}
