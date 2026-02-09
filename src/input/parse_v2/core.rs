use crate::input::contracts::tokens;

#[derive(Debug)]
pub enum ParserErrorDetails {
    IllegalImplementation(String),
    /// We consumed all tokens without seeing an EOF token.
    Incomplete,
    Other(String),
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
    pub fn seek(&mut self, i: usize) -> ParserResult<()> {
        let mkerr = |x: String| ParserError {
            location: self.location,
            details: ParserErrorDetails::IllegalImplementation(x),
        };
        if i < self.offset {
            return Err(mkerr(format!(
                "This should never happen! Attempted to seek to {i} even though offset is {}.",
                self.offset
            )));
        }
        if i >= (self.offset + self.tokens.len()) {
            return Err(
                mkerr(
                    format!(
                        "This should never happen! Attempted to seek to {i} even though there are only {} tokens ({} + {})",
                        self.offset + self.tokens.len(),
                        self.offset,
                        self.tokens.len()
                    )
                )
            );
        }
        self.location = i;

        Ok(())
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

pub trait Parser {
    type Output;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<Self::Output>;
}

impl<T> Parser for fn(&mut TokenScanner) -> ParserResult<T> {
    type Output = T;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<T> {
        self(scanner)
    }
}
