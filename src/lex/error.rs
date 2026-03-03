use crate::utils;

#[derive(Clone, Debug)]
pub enum LexerErrorDetails {
    NothingMatched,
    InternalError(String),
}

/// The main error type for the parser stage.
#[derive(Debug)]
pub struct LexerError {
    /// A reference to the original string we were trying to parse.
    pub content: std::sync::Arc<str>,
    /// The byte location of the token at which the error occurred.
    pub location: usize,
    /// Details about the exact error.
    pub details: LexerErrorDetails,
    pub previous_tokens: Vec<super::DecoratedToken>,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match &self.details {
            LexerErrorDetails::InternalError(_) => "Internal error.",
            LexerErrorDetails::NothingMatched => "Encountered invalid characters.",
        };

        let pos = utils::get_position_in_content(&self.content, self.location);
        writeln!(f, "Error at {}:{}", pos.0 + 1, pos.1 + 1)?;
        writeln!(
            f,
            "{}",
            utils::pretty_print_file_error(&self.content, self.location, msg)
        )?;

        writeln!(f, "Previous Tokens:")?;
        for token in self.previous_tokens.iter().rev().take(10) {
            // TODO: pretty print the tokens as well.
            writeln!(f, "  {:#?}", token.token())?;
        }

        Ok(())
    }
}
