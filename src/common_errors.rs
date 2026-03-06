//! A module to house error types commonly required in binaries, as well as their various trait implementations.
use crate::{lex, parsing, utils};
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub struct FileLexError {
    pub filename: Rc<PathBuf>,
    pub content: Rc<str>,
    pub error: lex::LexerError,
}

impl FileLexError {
    pub fn map_from_lexer_error(
        filename: Rc<PathBuf>,
        content: Rc<str>,
    ) -> impl FnOnce(lex::LexerError) -> Self {
        move |error| Self {
            filename,
            content,
            error,
        }
    }
}

impl std::fmt::Display for FileLexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, character) =
            utils::get_position_in_content(self.content.as_ref(), self.error.location);

        writeln!(
            f,
            "while formatting file: {}:{}:{}:",
            self.filename.as_os_str().to_str().unwrap(),
            line + 1,
            character + 1
        )?;
        writeln!(
            f,
            "{}",
            utils::pretty_print_location(self.content.as_ref(), self.error.location)
        )?;
        writeln!(f, "{}", self.error)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct FileParseError {
    pub filename: Rc<PathBuf>,
    pub content: Rc<str>,
    pub decorated_tokens: Rc<[lex::DecoratedToken]>,
    pub error: parsing::ParserError,
}

impl FileParseError {
    pub fn map_from_parser_error(
        filename: Rc<PathBuf>,
        content: Rc<str>,
        decorated_tokens: Rc<[lex::DecoratedToken]>,
    ) -> impl FnOnce(parsing::ParserError) -> Self {
        move |error| Self {
            filename,
            content,
            decorated_tokens,
            error,
        }
    }
}

impl std::fmt::Display for FileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "while parsing file: {}",
            self.filename.as_os_str().to_str().unwrap()
        )?;

        let errors = self.error.unwind();
        for (location, message) in errors {
            let content_location = self.decorated_tokens.get(location).unwrap().location();

            let (line, character) =
                utils::get_position_in_content(self.content.as_ref(), content_location);

            writeln!(f, "at {}:{}:", line + 1, character + 1)?;
            writeln!(
                f,
                "{}",
                utils::pretty_print_location(self.content.as_ref(), content_location)
            )?;

            writeln!(f, "{message}")?;
            writeln!(f)?;
        }
        Ok(())
    }
}
