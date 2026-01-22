use crate::input::{timestamp::Timestamp, *};

fn from_timestamp(s: &str) -> Timestamp {
    chrono::DateTime::parse_from_rfc3339(s).expect("Invalid timestamp")
}

pub fn test_001_empty_string() -> JournalAST {
    JournalAST(vec![])
}

pub fn transaction_001_simple() -> Transaction {
    Transaction {
        header: TransactionHeader {
            timestamp: from_timestamp("2026-01-01 00:00:00.000Z"),
        },
        postings: vec![],
    }
}
