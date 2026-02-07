use super::{Parser, ParserError, ParserResult, Token};
use crate::utils;
struct ManyParser<P> {
    parser: P,
}

impl<'a, T, P> Parser<'a> for ManyParser<P>
where
    P: Parser<'a, Output = T>,
{
    type Output = Vec<T>;

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

pub fn many<'a, P1, T>(parser: P1) -> impl Parser<'a, Output = Vec<T>>
where
    P1: Parser<'a, Output = T>,
{
    ManyParser { parser }
}

struct OptionalParser<P> {
    parser: P,
}

impl<'a, T, P> Parser<'a> for OptionalParser<P>
where
    P: Parser<'a, Output = T>,
{
    type Output = Option<T>;

    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, Option<T>> {
        match self.parser.parse(tokens) {
            Ok((rest, x)) => Ok((rest, Some(x))),
            Err(_) => Ok((tokens, None)),
        }
    }
}

pub fn optional<'a, P1, T>(parser: P1) -> impl Parser<'a, Output = Option<T>>
where
    P1: Parser<'a, Output = T>,
{
    OptionalParser { parser }
}

struct PrecededParser<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<'a, T, P1, P2> Parser<'a> for PrecededParser<P1, P2>
where
    P1: Parser<'a>,
    P2: Parser<'a, Output = T>,
{
    type Output = T;

    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, T> {
        let (tokens, _) = self.p1.parse(tokens)?;
        let (tokens, v) = self.p2.parse(tokens)?;
        Ok((tokens, v))
    }
}

pub fn preceded<'a, P1, P2, T>(p1: P1, p2: P2) -> impl Parser<'a, Output = T>
where
    P1: Parser<'a>,
    P2: Parser<'a, Output = T>,
{
    PrecededParser { p1, p2 }
}

struct TerminatedParser<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<'a, T, P1, P2> Parser<'a> for TerminatedParser<P1, P2>
where
    P1: Parser<'a, Output = T>,
    P2: Parser<'a>,
{
    type Output = T;

    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, T> {
        let (tokens, v) = self.p1.parse(tokens)?;
        let (tokens, _) = self.p2.parse(tokens)?;
        Ok((tokens, v))
    }
}

pub fn terminated<'a, P1, P2, T>(p1: P1, p2: P2) -> impl Parser<'a, Output = T>
where
    P1: Parser<'a, Output = T>,
    P2: Parser<'a>,
{
    TerminatedParser { p1, p2 }
}

struct OneOfParser<'a, P> {
    parsers: &'a [P],
}

impl<'a, T, P> Parser<'a> for OneOfParser<'_, P>
where
    P: Parser<'a, Output = T>,
{
    type Output = T;

    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, T> {
        let mut errors = vec![];

        for p in self.parsers.iter() {
            match p.parse(tokens) {
                Ok(x) => {
                    return Ok(x);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        let mut error_msg: String = "All parsers failed.".to_string();
        for error in &errors {
            error_msg += &utils::indent_string(&format!(
                "\nError:\n  {}",
                utils::indent_string(&error.message)
            ));
        }

        Err(ParserError::from_str(&error_msg))
    }
}

pub fn one_of<'a, P, T>(parsers: &'_ [P]) -> impl Parser<'a, Output = T> + '_
where
    P: Parser<'a, Output = T>,
{
    OneOfParser { parsers }
}

struct WithContextParser<P> {
    context: String,
    parser: P,
}

impl<'a, T, P> Parser<'a> for WithContextParser<P>
where
    P: Parser<'a, Output = T>,
{
    type Output = T;

    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, T> {
        match self.parser.parse(tokens) {
            Ok(x) => Ok(x),
            Err(e) => Err(ParserError::from_str(&format!(
                "{}: {}",
                self.context, e.message
            ))),
        }
    }
}

pub fn with_context<'a, P, T>(context: &str, parser: P) -> impl Parser<'a, Output = T>
where
    P: Parser<'a, Output = T>,
{
    WithContextParser {
        context: context.to_string(),
        parser,
    }
}

#[cfg(test)]
mod test {
    use super::super::parse_line_separator;
    use super::*;

    #[test]
    fn test_many_empty() {
        let tokens = [];
        let (rest, items) = many(parse_line_separator).parse(&tokens).expect("Failed.");
        assert!(items.is_empty());
        assert!(rest.is_empty());
    }

    #[test]
    fn test_many_not_matching() {
        let tokens = [Token::Indent];
        let (rest, items) = many(parse_line_separator).parse(&tokens).expect("Failed.");
        assert!(items.is_empty());
        assert!(!rest.is_empty());
    }

    #[test]
    fn test_one_of_simple() {
        let tokens = [Token::Indent];
        let parsers = [
            super::super::parse_indent,
            super::super::parse_line_separator,
        ];
        let (rest, items) = one_of(&parsers).parse(&tokens).expect("Failed.");
        assert!(rest.is_empty());
    }
}
