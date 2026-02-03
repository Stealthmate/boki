use chrono::{DateTime, FixedOffset};

pub type Timestamp = DateTime<FixedOffset>;

pub struct TransactionHeader {
    pub timestamp: Timestamp,
    pub attributes: serde_yaml::Mapping,
}

pub struct Posting {
    pub account: String,
    pub commodity: Option<String>,
    pub amount: Option<i64>,
}

pub struct Transaction {
    pub header: TransactionHeader,
    pub postings: Vec<Posting>,
}

pub enum ASTNode {
    Transaction(Transaction),
    SetAttribute(String, String),
}
