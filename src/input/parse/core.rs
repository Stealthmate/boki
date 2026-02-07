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

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

impl ParserError {
    pub fn from_str(s: &str) -> Self {
        ParserError {
            message: s.to_string(),
        }
    }
}

pub type ParserResult<'a, T> = Result<(&'a [Token], T), ParserError>;

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
        None => Err(ParserError::from_str("Unexpected EOF.")),
        Some(x) => Ok((&tokens[1..], x.clone())),
    }
}

macro_rules! parse_token {
    ($name:ident, $error_name:ident, $return_type:ty, $expansion:pat, $return_value:expr) => {
        pub fn $name(tokens: &[Token]) -> ParserResult<'_, $return_type> {
            let (rest, t) = next(tokens)?;
            let $expansion = t else {
                return Err(ParserError::from_str(&format!(
                    "Expected {} but found {}",
                    $error_name,
                    t.name()
                )));
            };

            Ok((rest, $return_value))
        }
    };
}

parse_token!(
    parse_timestamp,
    TOKEN_NAME_TIMESTAMP,
    Timestamp,
    Token::Timestamp(ts),
    ts
);
parse_token!(parse_amount, TOKEN_NAME_TIMESTAMP, i64, Token::Amount(x), x);
parse_token!(
    parse_identifier,
    TOKEN_NAME_TIMESTAMP,
    String,
    Token::Identifier(x),
    x
);
parse_token!(
    parse_account_separator,
    TOKEN_NAME_TIMESTAMP,
    (),
    Token::AccountSeparator,
    ()
);
parse_token!(
    parse_posting_separator,
    TOKEN_NAME_TIMESTAMP,
    (),
    Token::PostingSeparator,
    ()
);
parse_token!(
    parse_line_separator,
    TOKEN_NAME_TIMESTAMP,
    (),
    Token::LineSeparator,
    ()
);
parse_token!(
    parse_yaml_matter,
    TOKEN_NAME_TIMESTAMP,
    serde_yaml::Mapping,
    Token::YamlMatter(x),
    x
);
parse_token!(parse_indent, TOKEN_NAME_TIMESTAMP, (), Token::Indent, ());
parse_token!(parse_dedent, TOKEN_NAME_TIMESTAMP, (), Token::Dedent, ());

pub fn parse_keyword<'a>(tokens: &'a [Token], kw: Keyword) -> ParserResult<'a, ()> {
    let (rest, t) = next(tokens)?;
    match &t {
        Token::Keyword(x) if *x == kw => Ok((rest, ())),
        _ => Err(ParserError::from_str(&format!(
            "Expected {TOKEN_NAME_KEYWORD} but found {}",
            t.name()
        ))),
    }
}

mod combinators;
pub use combinators::{many, one_of, optional, preceded, terminated, with_context};
