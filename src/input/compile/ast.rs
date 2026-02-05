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

#[derive(Debug)]
pub enum CompilationError {
    GeneralError(String),
}

impl CompilationError {
    pub fn from_str(s: &str) -> Self {
        CompilationError::GeneralError(s.to_string())
    }
}

pub type CompilationResult<T> = Result<T, CompilationError>;
