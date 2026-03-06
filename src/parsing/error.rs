use crate::tokens;

#[derive(Debug)]
pub enum ParserErrorDetails {
    BranchingError(String, Vec<ParserError>),
    Nested(String, Box<ParserError>),
    ExpectedSomethingElse(String, tokens::Token),
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

impl ParserError {
    pub fn unwind(&self) -> Vec<(usize, String)> {
        let mut stack: Vec<(usize, String)> = vec![];
        match &self.details {
            ParserErrorDetails::BranchingError(msg, errors) => {
                stack.push((self.location, msg.clone()));
                for error in errors {
                    stack.extend_from_slice(&error.unwind());
                }
            }
            ParserErrorDetails::Nested(msg, error) => {
                stack.push((self.location, msg.clone()));
                stack.extend_from_slice(&error.unwind());
            }
            ParserErrorDetails::ExpectedSomethingElse(expected, found) => {
                stack.push((
                    self.location,
                    format!("Expected {expected} but found: {found}"),
                ));
            }
            ParserErrorDetails::IllegalImplementation(msg) => {
                stack.push((self.location, format!("Illegal implementation: {msg}")))
            }
            ParserErrorDetails::Incomplete => stack.push((self.location, "Incomplete".to_string())),
            ParserErrorDetails::Other(msg) => stack.push((self.location, format!("Other: {msg}"))),
        };

        stack
    }
}
