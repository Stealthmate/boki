#![allow(dead_code)]
#![allow(unused_variables)]

use crate::ast;
use crate::tokens;

mod error;
mod lex;
mod parse;

pub use error::LexParseError;
use error::{LexParseErrorContext, LexParseErrorDetails};

pub type LexParseResult<T> = Result<T, Box<LexParseError>>;

fn lexparse(context: LexParseErrorContext) -> LexParseResult<Vec<ast::ASTNode>> {
    let tokens = lex::lex_string(context.content()).map_err(|e| LexParseError {
        context: context.clone(),
        details: LexParseErrorDetails::Lex { details: e },
    })?;
    let the_tokens: Vec<tokens::Token> = tokens.iter().map(|x| x.token().clone()).collect();
    let mut scanner = parse::TokenScanner::from_slice(&the_tokens);
    let mut nodes = vec![];

    loop {
        let maybe_node = parse::parse_node(&mut scanner).map_err(|details| LexParseError {
            context: context.clone(),
            details: LexParseErrorDetails::Parse {
                tokens: tokens.clone(),
                details,
            },
        })?;

        let Some(node) = maybe_node else {
            break;
        };

        nodes.push(node);
    }

    Ok(nodes)
}

pub fn lexparse_string(content: &str) -> LexParseResult<Vec<ast::ASTNode>> {
    lexparse(LexParseErrorContext::NakedString {
        content: content.to_string(),
    })
}

/// Lexes and parses a single file.
pub fn lexparse_file(filename: &str) -> LexParseResult<Vec<ast::ASTNode>> {
    let content = std::fs::read_to_string(filename).unwrap();
    let context = LexParseErrorContext::File {
        filename: filename.to_string(),
        content,
    };
    lexparse(context)
}
