use crate::input::parse::syntax;

pub type ParserResult<'a, T> = Result<(&'a [syntax::Token], T), String>;

pub fn next(tokens: &[syntax::Token]) -> ParserResult<'_, syntax::Token> {
    match tokens.first() {
        None => Err("No more tokens!".to_string()),
        Some(x) => Ok((&tokens[1..], x.clone())),
    }
}
