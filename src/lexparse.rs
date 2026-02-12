#![allow(dead_code)]
#![allow(unused_variables)]

use crate::compile;
use crate::contracts::ast;

mod contracts;
mod error;
mod lex;
mod parse;

pub use error::InputError;

pub type InputResult<T> = Result<T, InputError>;

#[derive(Clone, Debug)]
pub enum LexParseErrorContext {
    File { filename: String, content: String },
    NakedString { content: String },
}

impl LexParseErrorContext {
    fn content(&self) -> &str {
        match self {
            Self::File { filename, content } => content,
            Self::NakedString { content } => content,
        }
    }
}

#[derive(Debug)]
pub enum LexParseErrorDetails {
    LexError {
        details: String, // TODO: proper type
    },
    ParseError {
        tokens: Vec<lex::core::DecoratedToken>,
        details: parse::core::ParserError,
    },
    OtherError(String),
}

#[derive(Debug)]
pub struct LexParseError {
    context: LexParseErrorContext,
    details: LexParseErrorDetails,
}
pub type LexParseResult<T> = Result<T, Box<LexParseError>>;

fn lexparse(context: LexParseErrorContext) -> LexParseResult<Vec<ast::ASTNode>> {
    let (_, tokens) = lex::lex_string(context.content()).map_err(|e| LexParseError {
        context: context.clone(),
        details: LexParseErrorDetails::LexError {
            details: e.to_string(),
        },
    })?;
    let the_tokens: Vec<contracts::tokens::Token> =
        tokens.iter().map(|x| x.token().clone()).collect();
    let mut scanner = parse::core::TokenScanner::from_slice(&the_tokens);
    let mut nodes = vec![];

    loop {
        let maybe_node = parse::parse_node(&mut scanner).map_err(|details| {
            // let xmin = if e.location > 10 {
            //     e.location - 10
            // } else {
            //     e.location
            // };
            // let xmax = std::cmp::min(e.location + 10, tokens.len());
            // let nearby_tokens = &tokens[xmin..xmax];
            // InputError::ParseError {
            //     filename: filename.to_string(),
            //     raw_content: content.to_string(),
            //     tokens: tokens.clone(),
            //     details: e,
            // }
            LexParseError {
                context: context.clone(),
                details: LexParseErrorDetails::ParseError {
                    tokens: tokens.clone(),
                    details,
                },
            }
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
