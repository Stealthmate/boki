#![allow(dead_code)]
#![allow(unused_variables)]

use crate::compile;
use crate::contracts::ast;
use crate::contracts::output;

mod contracts;
mod error;
mod lex;
mod parse;

pub use error::InputError;

pub type InputResult<T> = Result<T, InputError>;

fn parse_string(filename: &str, content: &str) -> InputResult<Vec<ast::ASTNode>> {
    let (_, tokens) = lex::lex_string(content).map_err(|e| InputError::LexError(e.to_string()))?;
    let the_tokens: Vec<contracts::tokens::Token> =
        tokens.iter().map(|x| x.token().clone()).collect();
    let mut scanner = parse::core::TokenScanner::from_slice(&the_tokens);
    let mut nodes = vec![];

    loop {
        let maybe_node = parse::parse_node(&mut scanner).map_err(|e| {
            let xmin = if e.location > 10 {
                e.location - 10
            } else {
                e.location
            };
            let xmax = std::cmp::min(e.location + 10, tokens.len());
            let nearby_tokens = &tokens[xmin..xmax];
            InputError::ParseError {
                filename: filename.to_string(),
                raw_content: content.to_string(),
                tokens: tokens.clone(),
                details: e,
            }
        })?;

        let Some(node) = maybe_node else {
            break;
        };

        nodes.push(node);
    }

    Ok(nodes)
}

fn parse_file(filename: &str) -> InputResult<Vec<ast::ASTNode>> {
    let content = std::fs::read_to_string(filename).unwrap();
    parse_string(filename, &content)
}

pub fn compile_file(filename: &str) -> InputResult<output::Journal> {
    let nodes = parse_file(filename)?;

    let mut journal = output::Journal::default();
    for node in nodes {
        compile::compile_node(&node, &mut journal).map_err(InputError::CompileError)?;
    }

    Ok(journal)
}

pub fn compile_string(content: &str) -> InputResult<output::Journal> {
    let nodes = parse_string("", content)?;

    let mut journal = output::Journal::default();
    for node in nodes {
        compile::compile_node(&node, &mut journal).map_err(InputError::CompileError)?;
    }

    Ok(journal)
}
