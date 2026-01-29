use crate::input::parse::syntax;

pub type ParserResult<'a, T> = Result<(&'a [syntax::Token], T), String>;

pub fn next(tokens: &[syntax::Token]) -> ParserResult<'_, syntax::Token> {
    match tokens.first() {
        None => Err("No more tokens!".to_string()),
        Some(x) => Ok((&tokens[1..], x.clone())),
    }
}

pub trait Parser<'a, T> {
    fn parse(&self, tokens: &'a [syntax::Token]) -> ParserResult<'a, T>;
}

impl<'a, T, F> Parser<'a, T> for F
where
    F: Fn(&'a [syntax::Token]) -> ParserResult<'a, T>,
{
    fn parse(&self, tokens: &'a [syntax::Token]) -> ParserResult<'a, T> {
        self(tokens)
    }
}

struct ManyParser<P> {
    parser: P,
}

impl<'a, T, P> Parser<'a, Vec<T>> for ManyParser<P>
where
    P: Parser<'a, T>,
{
    fn parse(&self, tokens: &'a [syntax::Token]) -> ParserResult<'a, Vec<T>> {
        todo!()
    }
}

pub fn many<'a, P1, T>(parser: P1) -> impl Parser<'a, Vec<T>>
where
    P1: Parser<'a, T>,
{
    ManyParser { parser }
}
