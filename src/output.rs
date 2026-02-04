#[derive(Clone, serde::Deserialize, Debug, PartialEq, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct JournalHeader {
    pub default_commodity: String,
}

#[derive(Clone, serde::Deserialize, Debug, PartialEq, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    pub account: String,
    pub commodity: String,
    pub amount: i64,
}

impl Default for Posting {
    fn default() -> Self {
        Posting {
            account: "".to_string(),
            commodity: "".to_string(),
            amount: 0,
        }
    }
}

pub type TransactionTimestamp = chrono::DateTime<chrono::FixedOffset>;

#[derive(Clone, serde::Deserialize, Debug, PartialEq, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionHeader {
    pub timestamp: TransactionTimestamp,
    pub attributes: serde_yaml::Mapping,
}

#[derive(Clone, serde::Deserialize, Debug, PartialEq, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    pub header: TransactionHeader,
    pub postings: Vec<Posting>,
}

#[derive(Clone, serde::Deserialize, Debug, PartialEq, serde::Serialize)]
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
