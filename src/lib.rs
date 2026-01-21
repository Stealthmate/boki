#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    account: String,
    commodity: String,
    amount: i64,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    timestamp: chrono::DateTime<chrono::FixedOffset>,
    postings: Vec<Posting>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Object {
    transactions: Vec<Transaction>,
}

pub fn compile_journal(path: &str) -> Object {
    Object {
        transactions: vec![Transaction {
            timestamp: chrono::DateTime::parse_from_rfc3339("2026-01-01T00:00:00.000Z").unwrap(),
            postings: vec![
                Posting {
                    account: "assets/cce/cash".to_string(),
                    commodity: "JPY".to_string(),
                    amount: -1000,
                },
                Posting {
                    account: "expense".to_string(),
                    commodity: "JPY".to_string(),
                    amount: 1000,
                },
            ],
        }],
    }
}

pub fn read_object(path: &str) -> Object {
    let file = std::fs::File::open(path).expect("Could not open file.");
    serde_json::from_reader(file).expect("Could not read file")
}
