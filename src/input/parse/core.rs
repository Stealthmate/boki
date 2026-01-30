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

pub type ParserResult<'a, T> = Result<(&'a [Token], T), String>;

pub trait Parser<'a> {
    type Output;

    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, Self::Output>;
}

impl<'a, T, F> Parser<'a> for F
where
    F: Fn(&'a [Token]) -> ParserResult<'a, T>,
{
    type Output = T;

    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, T> {
        self(tokens)
    }
}

pub fn next(tokens: &[Token]) -> ParserResult<'_, Token> {
    match tokens.first() {
        None => Err("No more tokens!".to_string()),
        Some(x) => Ok((&tokens[1..], x.clone())),
    }
}

macro_rules! parse_token {
    ($name:ident, $return_type:ty, $expansion:pat, $return_value:expr) => {
        pub fn $name(tokens: &[Token]) -> ParserResult<'_, $return_type> {
            let (rest, t) = next(tokens)?;
            let $expansion = t else {
                return Err("Wrong token".to_string());
            };

            Ok((rest, $return_value))
        }
    };
}

parse_token!(parse_timestamp, Timestamp, Token::Timestamp(ts), ts);
parse_token!(parse_amount, i64, Token::Amount(x), x);
parse_token!(parse_identifier, String, Token::Identifier(x), x);
parse_token!(parse_account_separator, (), Token::AccountSeparator, ());
parse_token!(parse_posting_separator, (), Token::PostingSeparator, ());
parse_token!(parse_line_separator, (), Token::LineSeparator, ());
parse_token!(parse_indent, (), Token::Indent, ());
parse_token!(parse_dedent, (), Token::Dedent, ());

mod combinators;
pub use combinators::{many, optional, preceded};
