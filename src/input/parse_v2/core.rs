use crate::input::contracts::tokens;

#[derive(Debug)]
pub enum ParserErrorDetails {
    /// We consumed all tokens without seeing an EOF token.
    Incomplete,
}

/// The main error type for the parser stage.
#[derive(Debug)]
pub struct ParserError {
    /// The location of the token at which the error occurred.
    pub location: usize,
    /// Details about the exact error.
    pub details: ParserErrorDetails,
}

pub type ParserResult<T> = Result<T, ParserError>;

pub struct TokenScanner {
    tokens: Vec<tokens::Token>,
    location: usize,
    offset: usize,
}

impl TokenScanner {
    pub fn from_slice(slice: &[tokens::Token]) -> Self {
        TokenScanner {
            tokens: slice.to_vec(),
            offset: 0,
            location: 0,
        }
    }
}

impl TokenScanner {
    pub fn tell(&self) -> usize {
        self.offset + self.location
    }
    pub fn seek(&mut self, i: usize) {
        todo!()
    }
    pub fn advance(&mut self, i: usize) {
        self.location += 1
    }
    pub fn peek(&self) -> Option<&tokens::Token> {
        self.tokens.get(self.location)
    }
}

pub fn peek_next(scanner: &mut TokenScanner) -> ParserResult<&tokens::Token> {
    match scanner.peek() {
        None => Err(ParserError {
            location: scanner.tell(),
            details: ParserErrorDetails::Incomplete,
        }),
        Some(t) => Ok(t),
    }
}
