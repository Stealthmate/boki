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

macro_rules! parse_token {
    ($name:ident, $return_type:ty, $expansion:pat, $return_value:expr) => {
        pub fn $name(tokens: &[Token]) -> core::ParserResult<'_, $return_type> {
            let (rest, t) = core::next(tokens)?;
            let $expansion = t else {
                return Err("Wrong token".to_string());
            };

            Ok((rest, $return_value))
        }
    };
}

parse_token!(parse_timestamp, Timestamp, Token::Timestamp(ts), ts);
parse_token!(parse_line_separator, (), Token::LineSeparator, ());
parse_token!(parse_indent, (), Token::Indent, ());
parse_token!(parse_dedent, (), Token::Dedent, ());
