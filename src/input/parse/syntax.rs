use crate::input::parse::core;
use chrono::{DateTime, FixedOffset};

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Clone, Debug, PartialEq)]
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

pub fn parse_timestamp(tokens: &[Token]) -> core::ParserResult<'_, Timestamp> {
    let (rest, t) = core::next(tokens)?;
    let Token::Timestamp(ts) = t else {
        return Err("Wrong token".to_string());
    };

    Ok((rest, ts))
}

pub fn parse_line_separator(tokens: &[Token]) -> core::ParserResult<'_, ()> {
    let (rest, t) = core::next(tokens)?;
    let Token::LineSeparator = t else {
        return Err("Wrong token".to_string());
    };

    Ok((rest, ()))
}
