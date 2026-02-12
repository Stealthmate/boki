use crate::lexparse::parse::core::{ParserError, ParserErrorDetails};

use crate::lexparse::lex;
use crate::lexparse::parse;

use crate::utils::indent_string;

#[derive(Clone, Debug)]
pub(super) enum LexParseErrorContext {
    File { filename: String, content: String },
    NakedString { content: String },
}

impl LexParseErrorContext {
    pub(super) fn content(&self) -> &str {
        match self {
            Self::File { filename, content } => content,
            Self::NakedString { content } => content,
        }
    }
}

#[derive(Debug)]
pub(crate) enum LexParseErrorDetails {
    Lex {
        details: String, // TODO: proper type
    },
    Parse {
        tokens: Vec<lex::core::DecoratedToken>,
        details: parse::core::ParserError,
    },
    Other(String),
}

#[derive(Debug)]
pub struct LexParseError {
    pub(super) context: LexParseErrorContext,
    pub(super) details: LexParseErrorDetails,
}

impl LexParseError {
    fn get_position_in_content(&self, location: usize) -> (usize, usize) {
        let mut i = 0;
        let mut last_line = 0;
        for (j, line) in self.context.content().split("\n").enumerate() {
            i += line.len() + 1;
            last_line = j;
            if i > location {
                break;
            }
        }

        (last_line, i - location)
    }

    fn get_location_string(&self) -> String {
        let mut s = String::new();
        match &self.context {
            LexParseErrorContext::File { filename, content } => {
                s += &format!("file {filename}:");
            }
            LexParseErrorContext::NakedString { content } => {
                s += "at position ";
            }
        };

        if let LexParseErrorDetails::Parse { tokens, details } = &self.details {
            println!("Token location: {}", details.location);
            let raw_loc = tokens
                .get(details.location)
                .map(|t| t.location())
                .unwrap_or(self.context.content().len());
            let p = self.get_position_in_content(raw_loc);
            s += &format!("{}:{}", p.0, p.1);
        };

        s
    }

    fn get_surrounding_lines(&self) -> Option<String> {
        match &self.details {
            LexParseErrorDetails::Parse { tokens, details } => {
                let raw_loc = tokens
                    .get(details.location)
                    .map(|t| t.location())
                    .unwrap_or(self.context.content().len());
                let p = self.get_position_in_content(raw_loc);
                let the_line = p.0;

                let offset = 5;
                let all_lines: Vec<&str> = self.context.content().split("\n").collect();

                let min_line = the_line.saturating_sub(offset);
                let max_line = if the_line + offset <= all_lines.len() {
                    the_line + offset
                } else {
                    all_lines.len()
                };

                let mut the_lines: Vec<String> = vec![];
                for (i, line) in all_lines[min_line..max_line].iter().enumerate() {
                    if the_line - min_line == i {
                        the_lines.push(format!("here -> {line}"));
                    } else {
                        the_lines.push(format!("        {line}"));
                    }
                }

                Some(the_lines.join("\n"))
            }
            _ => None,
        }
    }

    fn format_parse_error(error: &ParserError) -> String {
        let mut s = String::new();
        match &error.details {
            ParserErrorDetails::BranchingError(msg, errors) => {
                s += "Could not parse the remaining tokens.\n";
                for (i, e) in errors.iter().enumerate() {
                    let e_str = indent_string(&Self::format_parse_error(e));
                    s += &format!(
                        "  parser #{} failed because\n{}\n\n",
                        i + 1,
                        indent_string(&e_str)
                    );
                }
            }
            ParserErrorDetails::ExpectedSomethingElse(a, b) => {
                s += &format!("Expected {a}, but found a {}", b.name());
            }
            _ => {
                s += "TODO";
            }
        };

        s
    }
}

impl std::fmt::Display for LexParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LexParseError:")?;
        writeln!(f, "  in {}", self.get_location_string())?;

        if let Some(surrounding_lines) = self.get_surrounding_lines() {
            writeln!(f, "  =============")?;
            writeln!(f, "{}", indent_string(&surrounding_lines))?;
            writeln!(f, "  =============")?;
            writeln!(f)?
        }

        match &self.details {
            LexParseErrorDetails::Lex { details } => writeln!(f, "  {details}")?,
            LexParseErrorDetails::Parse { tokens, details } => {
                let x = indent_string(&Self::format_parse_error(details));
                writeln!(f, "  ParserError:\n{}", indent_string(&x))?;
            }
            LexParseErrorDetails::Other(x) => writeln!(f, "  {x}")?,
        };

        Ok(())
    }
}
