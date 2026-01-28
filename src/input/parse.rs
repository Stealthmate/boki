use chrono::{DateTime, FixedOffset};

use crate::input::compile::ast;

pub type Timestamp = DateTime<FixedOffset>;

#[derive(Debug, PartialEq)]
pub enum Token {
    Timestamp(Timestamp),
    Amount(i64),
    Identifier(String),
    AccountSeparator,
    PostingSeparator,
    Comment(String),
    Indent,
    Dedent,
}

type ParserError<'a> = nom_language::error::VerboseError<&'a [Token]>;

type ParserResult<'a, T> = nom::IResult<&'a [Token], T, ParserError<'a>>;

pub fn parse_tokens(tokens: &[Token]) -> ParserResult<'_, Vec<ast::ASTNode>> {
    Ok((tokens, vec![]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let (rest, result) = parse_tokens(&[]).expect("Failed.");
        assert!(result.is_empty());
        assert!(rest.is_empty());
    }
}
