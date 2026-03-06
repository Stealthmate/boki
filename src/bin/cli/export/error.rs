use boki::{common_errors, compile};
use std::path::PathBuf;
use std::rc::Rc;

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
    Lexer(common_errors::FileLexError),
    Parser(common_errors::FileParseError),
    Compiler(FileCompileError),
}

impl From<common_errors::FileLexError> for Box<Error> {
    fn from(value: common_errors::FileLexError) -> Self {
        Box::new(Error::Lexer(value))
    }
}

impl From<common_errors::FileParseError> for Box<Error> {
    fn from(value: common_errors::FileParseError) -> Self {
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
