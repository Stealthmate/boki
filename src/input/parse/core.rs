use crate::input::parse::syntax;

pub type ParserResult<'a, T> = Result<(&'a [syntax::Token], T), String>;
