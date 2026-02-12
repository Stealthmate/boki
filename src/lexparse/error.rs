use crate::lexparse::compile::CompilationError;
use crate::lexparse::lex::core::DecoratedToken;
use crate::lexparse::parse::core::{ParserError, ParserErrorDetails};

use crate::utils::indent_string;

fn compute_line_number(mut location: usize, tokens: &[DecoratedToken], content: &str) -> usize {
    if location == tokens.len() {
        location -= 1;
    }
    let initial_token_idx = tokens.get(location).unwrap().location();
    let mut i = 0;
    let mut last_line = 0;
    for (j, line) in content.split("\n").enumerate() {
        i += line.len();
        if i >= initial_token_idx {
            return j;
        }
        last_line = j;
    }

    last_line
}

fn format_nearby_lines(n: usize, content: &str) -> String {
    let offset = 3;
    let xmin = n.saturating_sub(offset);
    let lines: Vec<&str> = content.split("\n").collect();
    let xmax = std::cmp::min(n + offset, lines.len() - 1);
    let x = n - xmin;

    let lines = &lines[xmin..xmax];

    let mut output = String::new();
    if xmin > 0 {
        output += "  ...\n";
    }
    for (i, line) in lines.iter().enumerate() {
        if i == x {
            output += &format!("-> {}\n", line);
        } else {
            output += &format!("   {}\n", line);
        }
    }
    if xmax < content.len() {
        output += "  ...\n"
    }

    output
}

fn format_parse_error_details(
    f: &mut std::fmt::Formatter<'_>,
    details: &ParserErrorDetails,
) -> std::fmt::Result {
    write!(f, "  {:#?}", details)?;
    Ok(())
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
                writeln!(f, "  {}:{}:{}", filename, line_number, details.location)?;
                writeln!(f)?;
                writeln!(f, "  ===========")?;
                write!(f, "{}", indent_string(&nearby_lines))?;
                writeln!(f, "  ===========")?;
                writeln!(f)?;
                format_parse_error_details(f, &details.details)?;
            }
            InputError::CompileError(e) => {
                write!(f, "Compile Error:\n  {}", indent_string(&format!("{e:#?}")))?
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum InputError {
    LexError(String),
    ParseError {
        filename: String,
        raw_content: String,
        tokens: Vec<DecoratedToken>,
        details: ParserError,
    },
    CompileError(CompilationError),
}
