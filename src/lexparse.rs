#![allow(dead_code)]
#![allow(unused_variables)]

use crate::ast;
use crate::lex;
use crate::tokens;

mod error;
mod parse;

pub use error::LexParseError;
use error::{LexParseErrorContext, LexParseErrorDetails};

pub type LexParseResult<T> = Result<T, Box<LexParseError>>;

fn fold_tokens(
    mut a: Vec<lex::DecoratedToken>,
    t: lex::DecoratedToken,
) -> Vec<lex::DecoratedToken> {
    let Some(last) = a.last() else {
        a.push(t);
        return a;
    };

    match (last.token().name(), t.token().name()) {
        // consecutive newlines are combined into one
        (tokens::TOKEN_NAME_LINE_SEPARATOR, tokens::TOKEN_NAME_LINE_SEPARATOR) => a,
        // indent followed by newline is considered as a single newline
        (tokens::TOKEN_NAME_INDENT, tokens::TOKEN_NAME_LINE_SEPARATOR) => {
            let mut i = a.pop().unwrap().location();
            // The second-to-last token could be a newline. In that case, we pop that as well.
            if a.last()
                .map(|t| matches!(t.token(), tokens::Token::LineSeparator))
                .unwrap_or(false)
            {
                i = a.pop().unwrap().location();
            }

            // Finally we put a newline at the end.
            a.push(lex::DecoratedToken::new(tokens::Token::LineSeparator, i));
            a
        }
        // Indent not following a newline is skipped
        (n, tokens::TOKEN_NAME_INDENT) if n != tokens::TOKEN_NAME_LINE_SEPARATOR => a,
        _ => {
            a.push(t);
            a
        }
    }
}

fn lexparse(context: LexParseErrorContext) -> LexParseResult<Vec<ast::ASTNode>> {
    let tokens = lex::lex_string(context.content()).map_err(|e| LexParseError {
        context: context.clone(),
        details: LexParseErrorDetails::Lex { details: e },
    })?;
    let tokens = tokens.into_iter().fold(vec![], fold_tokens);
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
