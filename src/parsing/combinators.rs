use crate::parsing::core::{get_next, Parser, ParserResult, TokenScanner};
use crate::parsing::error;
use crate::tokens;

struct ManyParser<P> {
    parser: P,
}

impl<T, P> Parser for ManyParser<P>
where
    P: Parser<Output = T>,
{
    type Output = Vec<T>;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<Vec<T>> {
        let mut parsed = vec![];

        loop {
            let i = scanner.tell();
            match self.parser.parse(scanner) {
                Err(_) => {
                    scanner.seek(i)?;
                    break;
                }
                Ok(x) => {
                    parsed.push(x);
                }
            }
        }

        Ok(parsed)
    }
}

pub fn many<P1, T>(parser: P1) -> impl Parser<Output = Vec<T>>
where
    P1: Parser<Output = T>,
{
    ManyParser { parser }
}

struct OptionalParser<P> {
    parser: P,
}

impl<T, P> Parser for OptionalParser<P>
where
    P: Parser<Output = T>,
{
    type Output = Option<T>;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<Option<T>> {
        let i = scanner.tell();
        match self.parser.parse(scanner) {
            Ok(x) => Ok(Some(x)),
            Err(_) => {
                scanner.seek(i)?;
                Ok(None)
            }
        }
    }
}

pub fn optional<P1, T>(parser: P1) -> impl Parser<Output = Option<T>>
where
    P1: Parser<Output = T>,
{
    OptionalParser { parser }
}

struct PrecededParser<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<T, P1, P2> Parser for PrecededParser<P1, P2>
where
    P1: Parser,
    P2: Parser<Output = T>,
{
    type Output = T;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<T> {
        self.p1.parse(scanner)?;
        let x = self.p2.parse(scanner)?;
        Ok(x)
    }
}

pub fn preceded<P1, P2, T>(p1: P1, p2: P2) -> impl Parser<Output = T>
where
    P1: Parser,
    P2: Parser<Output = T>,
{
    PrecededParser { p1, p2 }
}

struct TerminatedParser<P1, P2> {
    p1: P1,
    p2: P2,
}

impl<T, P1, P2> Parser for TerminatedParser<P1, P2>
where
    P1: Parser<Output = T>,
    P2: Parser,
{
    type Output = T;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<T> {
        let x = self.p1.parse(scanner)?;
        self.p2.parse(scanner)?;
        Ok(x)
    }
}

pub fn terminated<P1, P2, T>(p1: P1, p2: P2) -> impl Parser<Output = T>
where
    P1: Parser<Output = T>,
    P2: Parser,
{
    TerminatedParser { p1, p2 }
}

struct OneOfParser<'a, P> {
    parsers: &'a [P],
}

impl<T, P> Parser for OneOfParser<'_, P>
where
    P: Parser<Output = T>,
{
    type Output = T;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<T> {
        let i = scanner.tell();
        let mut errors = vec![];
        for p in self.parsers.iter() {
            scanner.seek(i)?;

            match p.parse(scanner) {
                Ok(x) => return Ok(x),
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        Err(error::ParserError {
            location: i,
            details: error::ParserErrorDetails::BranchingError(
                "All parsers failed.".to_string(),
                errors,
            ),
        })
    }
}

pub fn one_of<P, T>(parsers: &'_ [P]) -> impl Parser<Output = T> + '_
where
    P: Parser<Output = T>,
{
    OneOfParser { parsers }
}

struct TakeUntilParser<P> {
    parser: P,
    inclusive: bool,
}

impl<P> Parser for TakeUntilParser<P>
where
    P: Parser,
{
    type Output = Vec<tokens::Token>;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<Self::Output> {
        let mut tokens = vec![];
        loop {
            let i = scanner.tell();
            let result = self.parser.parse(scanner);
            scanner.seek(i)?;
            match result {
                Ok(_) => {
                    if self.inclusive {
                        scanner.advance(1)?;
                    }
                    break;
                }
                Err(_) => {
                    let t = get_next(scanner)?;
                    tokens.push(t.clone());
                }
            }
        }

        Ok(tokens)
    }
}

pub fn take_until<P>(parser: P, inclusive: bool) -> impl Parser<Output = Vec<tokens::Token>>
where
    P: Parser,
{
    TakeUntilParser { parser, inclusive }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsing::error::{ParserError, ParserErrorDetails};
    use crate::tokens;

    fn parse_fail(_: &mut TokenScanner) -> ParserResult<()> {
        Err(ParserError {
            location: 0,
            details: ParserErrorDetails::Incomplete,
        })
    }

    fn parse_succeed(_: &mut TokenScanner) -> ParserResult<()> {
        Ok(())
    }

    #[test]
    fn test_one_of_first() {
        let mut scanner = TokenScanner::from_slice(&[tokens::Token::Indent]);
        let parsers = [parse_succeed, parse_fail];
        one_of(&parsers).parse(&mut scanner).expect("Failed.");
    }

    #[test]
    fn test_one_of_second() {
        let mut scanner = TokenScanner::from_slice(&[tokens::Token::Indent]);
        let parsers = [parse_fail, parse_succeed];
        one_of(&parsers).parse(&mut scanner).expect("Failed.");
    }
}
