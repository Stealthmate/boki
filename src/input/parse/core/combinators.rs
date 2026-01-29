use super::{Parser, ParserResult, Token};
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

struct OptionalParser<P> {
    parser: P,
}

impl<'a, T, P> Parser<'a, Option<T>> for OptionalParser<P>
where
    P: Parser<'a, T>,
{
    fn parse(&self, tokens: &'a [Token]) -> ParserResult<'a, Option<T>> {
        match self.parser.parse(tokens) {
            Ok((rest, x)) => Ok((rest, Some(x))),
            Err(_) => Ok((tokens, None)),
        }
    }
}

pub fn optional<'a, P1, T>(parser: P1) -> impl Parser<'a, Option<T>>
where
    P1: Parser<'a, T>,
{
    OptionalParser { parser }
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
        let tokens = [Token::LineSeparator];
        let (rest, items) = many(parse_line_separator).parse(&tokens).expect("Failed.");
        assert!(items.is_empty());
        assert!(!rest.is_empty());
    }
}
