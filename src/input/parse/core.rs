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

pub trait Parser<'a, T> {
    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, T>;
}

impl<'a, T, F> Parser<'a, T> for F
where
    F: Fn(&'a [Token]) -> ParserResult<'a, T>,
{
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
parse_token!(parse_line_separator, (), Token::LineSeparator, ());
parse_token!(parse_indent, (), Token::Indent, ());
parse_token!(parse_dedent, (), Token::Dedent, ());

struct ManyParser<P> {
    parser: P,
}

impl<'a, T, P> Parser<'a, Vec<T>> for ManyParser<P>
where
    P: Parser<'a, T>,
{
    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, Vec<T>> {
        let mut parsed = vec![];
        let mut rest = tokens;

        loop {
            match self.parser.parse(rest) {
                Err(_) => {
                    break;
                }
                Ok((next_rest, next_parsed)) => {
                    rest = next_rest;
                    parsed.push(next_parsed);
                }
            }
        }

        Ok((rest, parsed))
    }
}

pub fn many<'a, P1, T>(parser: P1) -> impl Parser<'a, Vec<T>>
where
    P1: Parser<'a, T>,
{
    ManyParser { parser }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_many_empty() {
        let tokens = [];
        let (rest, items) = many(parse_timestamp).parse(&tokens).expect("Failed.");
        assert!(items.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_many_not_matching() {
        let tokens = [Token::Indent];
        let (rest, items) = many(parse_timestamp).parse(&tokens).expect("Failed.");
        assert!(items.is_empty());
        assert!(!rest.is_empty());
    }
}
