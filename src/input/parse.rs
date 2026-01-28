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

impl Token {
    pub fn is_comment(&self) -> bool {
        matches!(self, Token::Comment(_))
    }
}

type ParserResult<'a, T> = Result<(&'a [Token], T), String>;

fn parse_comments(tokens: &[Token]) -> ParserResult<'_, ()> {
    let mut rest = tokens;

    loop {
        if !matches!(rest.first(), Some(x) if x.is_comment()) {
            break;
        }
        rest = &rest[1..];
    }

    Ok((rest, ()))
}

pub fn parse_tokens(tokens: &[Token]) -> ParserResult<'_, Vec<ast::ASTNode>> {
    let (tokens, _) = parse_comments(tokens)?;
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

    #[test]
    fn test_only_comments() {
        let tokens = [
            Token::Comment("foo".to_string()),
            Token::Comment("foo".to_string()),
        ];
        let (rest, result) = parse_tokens(&tokens).expect("Failed.");
        assert!(result.is_empty());
        assert!(rest.is_empty());
    }
}
