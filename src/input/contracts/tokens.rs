use chrono::{DateTime, FixedOffset};

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Set,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Timestamp(Timestamp),
    Amount(i64),
    Identifier(String),
    AccountSeparator,
    PostingSeparator,
    LineSeparator,
    Comment(String),
    YamlMatter(serde_yaml::Mapping),
    Indent,
    Dedent,
}

pub const TOKEN_NAME_KEYWORD: &str = "keyword";
pub const TOKEN_NAME_TIMESTAMP: &str = "timestamp";
pub const TOKEN_NAME_AMOUNT: &str = "amount";
pub const TOKEN_NAME_IDENTIFIER: &str = "identifier";

impl Token {
    pub fn is_comment(&self) -> bool {
        matches!(self, Token::Comment(_))
    }

    pub fn name(&self) -> &'static str {
        match self {
            Token::Keyword(_) => TOKEN_NAME_KEYWORD,
            Token::Timestamp(_) => TOKEN_NAME_TIMESTAMP,
            Token::Amount(_) => TOKEN_NAME_AMOUNT,
            _ => "todo",
        }
    }
}
