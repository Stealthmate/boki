#![allow(dead_code)]
#![allow(unused_variables)]

use crate::input::contracts::ast;
use crate::utils::indent_string;

mod compile;
mod contracts;
mod lex;
mod parse;
mod parse_v2;

#[derive(Debug)]
pub enum InputError {
    LexError(String),
    ParseError {
        filename: String,
        raw_content: String,
        tokens: Vec<lex::core::DecoratedToken>,
        details: parse_v2::core::ParserError,
    },
    CompileError(compile::CompilationError),
}

fn compute_line_number(
    location: usize,
    tokens: &[lex::core::DecoratedToken],
    content: &str,
) -> usize {
    let initial_token_idx = tokens.get(location).unwrap().location();
    let mut i = 0;
    for (j, line) in content.split("\n").enumerate() {
        i += line.len();
        if i >= initial_token_idx {
            return j;
        }
    }

    0
}

fn format_nearby_lines(n: usize, content: &str) -> String {
    let offset = 3;
    let xmin = n.saturating_sub(offset);
    let xmax = n + offset;
    let x = n - xmin;

    let lines = &content.split("\n").collect::<Vec<&str>>()[xmin..xmax];

    let mut output = String::new();
    for (i, line) in lines.iter().enumerate() {
        if i == x {
            output += &format!("-> {}\n", line);
        } else {
            output += &format!("   {}\n", line);
        }
    }

    output
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            InputError::LexError(e) => write!(f, "Lex Error:\n  {}", indent_string(e))?,
            InputError::ParseError {
                filename,
                raw_content,
                tokens,
                details,
            } => {
                let line_number = compute_line_number(details.location, tokens, raw_content);
                let nearby_lines = format_nearby_lines(line_number, raw_content);
                writeln!(f, "Parse Error:")?;
                write!(f, "  {}:{}:{}\n\n", filename, line_number, details.location)?;
                write!(f, "  {}\n\n", indent_string(&nearby_lines))?;
                write!(f, "  {:#?}", details.details)?;
            }
            InputError::CompileError(e) => {
                write!(f, "Compile Error:\n  {}", indent_string(&format!("{e:#?}")))?
            }
        };

        Ok(())
    }
}

pub type InputResult<T> = Result<T, InputError>;

fn parse_file(filename: &str) -> InputResult<Vec<ast::ASTNode>> {
    let content = std::fs::read_to_string(filename).unwrap();
    let (_, tokens) = lex::lex_string(&content).map_err(|e| InputError::LexError(e.to_string()))?;
    let the_tokens: Vec<contracts::tokens::Token> =
        tokens.iter().map(|x| x.token().clone()).collect();
    let mut scanner = parse_v2::core::TokenScanner::from_slice(&the_tokens);
    let mut nodes = vec![];

    loop {
        let maybe_node = parse_v2::parse_node(&mut scanner).map_err(|e| {
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

pub fn compile_file(filename: &str) -> InputResult<crate::output::Journal> {
    let nodes = parse_file(filename)?;

    let mut journal = crate::output::Journal::default();
    for node in nodes {
        compile::compile_node(&node, &mut journal).map_err(InputError::CompileError)?;
    }

    Ok(journal)
}
