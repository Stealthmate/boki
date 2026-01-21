#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Posting {
    pub account: String,
    pub commodity: String,
    pub amount: i64,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    pub timestamp: chrono::DateTime<chrono::FixedOffset>,
    pub postings: Vec<Posting>,
}

#[derive(serde::Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub transactions: Vec<Transaction>,
}

pub fn read_object(path: &str) -> Object {
    let file = std::fs::File::open(path).expect("Could not open file.");
    serde_json::from_reader(file).expect("Could not read file")
}
