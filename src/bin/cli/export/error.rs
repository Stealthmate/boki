use boki::{lex, parsing, utils};
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub struct FileLexError {
    pub filename: Rc<PathBuf>,
    pub content: Rc<str>,
    pub error: lex::LexerError,
}

impl std::fmt::Display for FileLexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, character) =
            boki::utils::get_position_in_content(self.content.as_ref(), self.error.location);

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

impl std::fmt::Display for FileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "while formatting file: {}",
            self.filename.as_os_str().to_str().unwrap()
        )?;

        let errors = self.error.unwind();
        for (location, message) in errors {
            let content_location = self.decorated_tokens.get(location).unwrap().location();

            let (line, character) =
                boki::utils::get_position_in_content(self.content.as_ref(), content_location);

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

#[derive(Debug)]
pub enum Error {
    IO {
        filename: Rc<PathBuf>,
        error: std::io::Error,
    },
    Lexer(FileLexError),
    Parser(FileParseError),
}

impl From<FileLexError> for Box<Error> {
    fn from(value: FileLexError) -> Self {
        Box::new(Error::Lexer(value))
    }
}

impl From<FileParseError> for Box<Error> {
    fn from(value: FileParseError) -> Self {
        Box::new(Error::Parser(value))
    }
}

impl crate::error::CLIError for Error {
    fn format(&self) -> String {
        match &self {
            Self::Lexer(e) => format!("{e}"),
            Self::Parser(e) => format!("{e}"),
            _ => todo!(),
        }
    }
}
