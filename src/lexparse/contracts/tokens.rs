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
    Eof,
}

pub const TOKEN_NAME_KEYWORD: &str = "keyword";
pub const TOKEN_NAME_TIMESTAMP: &str = "timestamp";
pub const TOKEN_NAME_AMOUNT: &str = "amount";
pub const TOKEN_NAME_IDENTIFIER: &str = "identifier";
pub const TOKEN_NAME_ACCOUNT_SEPARATOR: &str = "account separator";
pub const TOKEN_NAME_POSTING_SEPARATOR: &str = "posting separator";
pub const TOKEN_NAME_LINE_SEPARATOR: &str = "line separator";
pub const TOKEN_NAME_COMMENT: &str = "comment";
pub const TOKEN_NAME_YAML_MATTER: &str = "YAML matter";
pub const TOKEN_NAME_INDENT: &str = "indent";
pub const TOKEN_NAME_EOF: &str = "eof";

impl Token {
    pub fn is_comment(&self) -> bool {
        matches!(self, Token::Comment(_))
    }

    pub fn name(&self) -> &'static str {
        match self {
            Token::Keyword(_) => TOKEN_NAME_KEYWORD,
            Token::Timestamp(_) => TOKEN_NAME_TIMESTAMP,
            Token::Amount(_) => TOKEN_NAME_AMOUNT,
            Token::Identifier(_) => TOKEN_NAME_IDENTIFIER,
            Token::AccountSeparator => TOKEN_NAME_ACCOUNT_SEPARATOR,
            Token::PostingSeparator => TOKEN_NAME_POSTING_SEPARATOR,
            Token::LineSeparator => TOKEN_NAME_LINE_SEPARATOR,
            Token::Comment(_) => TOKEN_NAME_COMMENT,
            Token::YamlMatter(_) => TOKEN_NAME_YAML_MATTER,
            Token::Indent => TOKEN_NAME_INDENT,
            Token::Eof => TOKEN_NAME_EOF,
        }
    }
}
