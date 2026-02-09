#![allow(dead_code)]
#![allow(unused_variables)]
use crate::output;
use crate::utils::indent_string;

mod compile;
mod contracts;
mod lex;
mod parse;
mod parse_v2;

#[derive(Debug)]
pub enum InputError {
    LexError(String),
    ParseError(String),
    CompileError(compile::CompilationError),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InputError::LexError(e) => write!(f, "Lex Error:\n  {}", indent_string(e))?,
            InputError::ParseError(e) => write!(f, "Parse Error:\n  {}", indent_string(e))?,
            InputError::CompileError(e) => {
                write!(f, "Compile Error:\n  {}", indent_string(&format!("{e:#?}")))?
            }
        };

        Ok(())
    }
}

pub type InputResult<T> = Result<T, InputError>;

pub fn compile(nodes: Vec<contracts::ast::ASTNode>) -> compile::CompilationResult<output::Journal> {
    let mut journal = output::Journal::default();

    for node in &nodes {
        compile::compile_node(node, &mut journal)?;
    }

    Ok(journal)
}

pub fn compile_string(input: &str) -> InputResult<crate::output::Journal> {
    let (_, tokens) = lex::lex_string(input).map_err(|e| InputError::LexError(e.to_string()))?;
    let (_, ast) = parse::parse_tokens(&tokens).map_err(InputError::ParseError)?;
    compile(ast).map_err(InputError::CompileError)
}
