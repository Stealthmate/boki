use chrono::{DateTime, FixedOffset};

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Timestamp(Timestamp),
    Amount(i64),
    Identifier(String),
    AccountSeparator,
    PostingSeparator,
    LineSeparator,
    Comment(String),
    Indent,
    Dedent,
}

impl Token {
    pub fn is_comment(&self) -> bool {
        matches!(self, Token::Comment(_))
    }
}
