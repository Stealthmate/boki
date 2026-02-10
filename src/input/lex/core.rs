use crate::input::contracts::tokens::Token;

pub type LexResult<'a, T> = nom::IResult<&'a str, T, nom_language::error::VerboseError<&'a str>>;

pub type TokenLocation = usize;

#[derive(Clone, Debug)]
pub struct DecoratedToken(Token, TokenLocation);

impl DecoratedToken {
    pub fn new(t: Token, i: TokenLocation) -> Self {
        Self(t, i)
    }

    pub fn token(&self) -> &Token {
        &self.0
    }

    pub fn location(&self) -> usize {
        self.1
    }
}
