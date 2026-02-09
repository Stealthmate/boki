use chrono::{DateTime, FixedOffset};

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Clone)]
pub struct TransactionHeader {
    pub timestamp: Timestamp,
    pub attributes: serde_yaml::Mapping,
}

#[derive(Clone)]
pub struct Posting {
    pub account: String,
    pub commodity: Option<String>,
    pub amount: Option<i64>,
}

#[derive(Clone)]
pub struct Transaction {
    pub header: TransactionHeader,
    pub postings: Vec<Posting>,
}

#[derive(Clone)]
pub enum ASTNode {
    Transaction(Transaction),
    SetAttribute(String, String),
}
