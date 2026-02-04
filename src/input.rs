#![allow(dead_code)]
#![allow(unused_variables)]

mod compile;
mod lex;
mod parse;

#[derive(Debug)]
pub enum InputError {
    LexError(String),
    ParseError(String),
    CompileError(compile::ast::CompilationError),
}

pub type InputResult<T> = Result<T, InputError>;

pub fn compile_string(input: &str) -> InputResult<crate::output::Journal> {
    let (_, tokens) = lex::lex_string(input).map_err(|e| InputError::LexError(e.to_string()))?;
    let (_, ast) = parse::parse_tokens(&tokens).map_err(InputError::ParseError)?;
    compile::compile(ast).map_err(InputError::CompileError)
}
