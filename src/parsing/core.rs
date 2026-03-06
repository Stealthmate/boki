use super::error;
use crate::tokens;

pub type ParserResult<T> = Result<T, error::ParserError>;

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
        let mkerr = |x: String| error::ParserError {
            location: self.location,
            details: error::ParserErrorDetails::IllegalImplementation(x),
        };
        if i < self.offset {
            return Err(mkerr(format!(
                "This should never happen! Attempted to seek to {i} even though offset is {}.",
                self.offset
            )));
        }
        self.location = i - self.offset;

        Ok(())
    }
    pub fn advance(&mut self, i: usize) -> ParserResult<()> {
        self.seek(self.location + i)
    }
    pub fn peek(&self) -> Option<&tokens::Token> {
        self.tokens.get(self.location)
    }

    // TODO:
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&tokens::Token> {
        let t = self.tokens.get(self.location);
        self.location += 1;
        t
    }

    pub fn tokens(&self) -> &[tokens::Token] {
        &self.tokens[self.offset..]
    }
}

pub fn peek_next(scanner: &TokenScanner) -> ParserResult<&tokens::Token> {
    match scanner.peek() {
        None => Err(error::ParserError {
            location: scanner.tell(),
            details: error::ParserErrorDetails::Incomplete,
        }),
        Some(t) => Ok(t),
    }
}

pub fn get_next(scanner: &mut TokenScanner) -> ParserResult<&tokens::Token> {
    let location = scanner.tell();
    match scanner.next() {
        None => Err(error::ParserError {
            location,
            details: error::ParserErrorDetails::Incomplete,
        }),
        Some(t) => Ok(t),
    }
}

pub trait Parser {
    type Output;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<Self::Output>;
}

impl<T, F> Parser for F
where
    F: Fn(&mut TokenScanner) -> ParserResult<T>,
{
    type Output = T;

    fn parse(&self, scanner: &mut TokenScanner) -> ParserResult<T> {
        self(scanner)
    }
}
