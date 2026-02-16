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
        details: lex::LexerError,
    },
    Parse {
        tokens: Vec<lex::DecoratedToken>,
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
        println!("str location: {location}");
        let mut i = 0;
        let mut last_line = 0;
        for (j, line) in self.context.content().split("\n").enumerate() {
            last_line = j;
            let start_of_next_line = i + line.len() + 1;
            if start_of_next_line > location {
                break;
            }
            i = start_of_next_line;
        }

        (last_line, location - i)
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

        match &self.details {
            LexParseErrorDetails::Parse { tokens, details } => {
                println!("Token location: {}", details.location);
                let raw_loc = tokens
                    .get(details.location)
                    .map(|t| t.location())
                    .unwrap_or(self.context.content().len());
                let p = self.get_position_in_content(raw_loc);
                s += &format!("{}:{}", p.0, p.1);
            }
            LexParseErrorDetails::Lex { details } => {
                let p = self.get_position_in_content(details.location);
                s += &format!("{}:{}", p.0, p.1);
            }
            _ => {}
        };

        s
    }

    fn arrow_to(&self, line: &str, charloc: usize) -> Vec<String> {
        let mut lines = vec![];
        println!("charloc {charloc}");
        lines.push(format!("        {}Λ", " ".repeat(charloc)));
        lines.push(format!("        {}│", " ".repeat(charloc)));
        lines.push(format!("  here  {}┘", "─".repeat(charloc)));

        lines
    }

    fn get_surrounding_lines_for_line(&self, n: usize, charloc: usize) -> String {
        let offset = 5;
        let all_lines: Vec<&str> = self.context.content().split("\n").collect();

        let min_line = n.saturating_sub(offset);
        let max_line = if n + offset <= all_lines.len() {
            n + offset
        } else {
            all_lines.len()
        };

        let mut the_lines: Vec<String> = vec![];
        for (i, line) in all_lines[min_line..max_line].iter().enumerate() {
            if i == n - min_line {
                the_lines.push(format!("        {line}"));
                the_lines.extend(self.arrow_to(line, charloc));
            } else {
                the_lines.push(format!("        {line}"));
            }
        }

        the_lines.join("\n")
    }

    fn get_surrounding_lines(&self) -> Option<String> {
        match &self.details {
            LexParseErrorDetails::Parse { tokens, details } => {
                let raw_loc = tokens
                    .get(details.location)
                    .map(|t| t.location())
                    .unwrap_or(self.context.content().len());
                let p = self.get_position_in_content(raw_loc);
                Some(self.get_surrounding_lines_for_line(p.0, p.1))
            }
            LexParseErrorDetails::Lex { details } => {
                let p = self.get_position_in_content(details.location);
                Some(self.get_surrounding_lines_for_line(p.0, p.1))
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

    fn format_lex_error(error: &lex::LexerError) -> String {
        let mut s = String::new();
        match &error.details {
            lex::LexerErrorDetails::InternalError(_) => {
                s += "Internal error.";
            }
            lex::LexerErrorDetails::NothingMatched => {
                s += "Encountered invalid characters. Previous tokens: ";
                s += &error
                    .previous_tokens
                    .iter()
                    .map(|t| format!("({}, {})", t.location(), t.token().name()))
                    .collect::<Vec<String>>()
                    .join(", ");
            }
        }
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
            LexParseErrorDetails::Lex { details } => {
                let x = indent_string(&Self::format_lex_error(details));
                writeln!(f, "  Lexer error:\n{}", indent_string(&x))?
            }
            LexParseErrorDetails::Parse { tokens, details } => {
                let x = indent_string(&Self::format_parse_error(details));
                writeln!(f, "  ParserError:\n{}", indent_string(&x))?;
            }
            LexParseErrorDetails::Other(x) => writeln!(f, "  {x}")?,
        };

        Ok(())
    }
}
