use boki::{compile, lex, parsing, utils};
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub struct FileLexError {
    pub filename: Rc<PathBuf>,
    pub content: Rc<str>,
    pub error: lex::LexerError,
}

pub fn map_lexer_error(
    filename: Rc<PathBuf>,
    content: Rc<str>,
) -> impl FnOnce(lex::LexerError) -> FileLexError {
    move |error| FileLexError {
        filename,
        content,
        error,
    }
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

pub fn map_parser_error(
    filename: Rc<PathBuf>,
    content: Rc<str>,
    decorated_tokens: Rc<[lex::DecoratedToken]>,
) -> impl FnOnce(parsing::ParserError) -> FileParseError {
    move |error| FileParseError {
        filename,
        content,
        decorated_tokens,
        error,
    }
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
pub struct FileCompileError {
    error: compile::CompilationError,
}

impl std::fmt::Display for FileCompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO: {:#?}", self.error)
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
    Compiler(FileCompileError),
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

pub fn map_io_error(filename: Rc<PathBuf>) -> impl FnOnce(std::io::Error) -> Box<Error> {
    move |error| Box::new(Error::IO { filename, error })
}

pub fn map_compile_error() -> impl FnOnce(compile::CompilationError) -> Box<Error> {
    |error| Box::new(Error::Compiler(FileCompileError { error }))
}

pub fn map_serde_error() -> impl FnOnce(serde_json::Error) -> Box<Error> {
    |_| todo!()
}
