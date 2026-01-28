#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct JournalHeader {
    pub default_commodity: String,
}

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
    pub header: JournalHeader,
    pub transactions: Vec<Transaction>,
}

impl Default for Journal {
    fn default() -> Self {
        Journal {
            header: JournalHeader {
                default_commodity: "".to_string(),
            },
            transactions: vec![],
        }
    }
}
