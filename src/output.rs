#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    pub account: String,
    pub commodity: String,
    pub amount: i64,
}

pub type TransactionTimestamp = chrono::DateTime<chrono::FixedOffset>;

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TransactionHeader {
    pub timestamp: TransactionTimestamp,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    pub header: TransactionHeader,
    pub postings: Vec<Posting>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Journal {
    pub transactions: Vec<Transaction>,
}
