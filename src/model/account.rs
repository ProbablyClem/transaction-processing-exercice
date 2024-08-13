use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}
