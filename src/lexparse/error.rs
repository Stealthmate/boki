use crate::lex;
use crate::lexparse::parse::{ParserError, ParserErrorDetails};

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
        details: parse::ParserError,
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

    fn append_error(
        &self,
        f: &mut std::fmt::Formatter,
        location: usize,
        msg: &str,
    ) -> std::fmt::Result {
        write!(f, "in ")?;

        match &self.context {
            LexParseErrorContext::File { filename, content } => {
                write!(f, "file {filename}:")?;
            }
            LexParseErrorContext::NakedString { content } => {
                write!(f, "at position ")?;
            }
        };

        let p = self.get_position_in_content(location);
        writeln!(f, "{}:{}", p.0, p.1)?;

        let surrounding_lines = self.get_surrounding_lines_for_line(p.0, p.1);
        writeln!(f, "  =============")?;
        writeln!(f, "{}", indent_string(&surrounding_lines))?;
        writeln!(f, "  =============")?;
        writeln!(f, "  {}", msg)?;
        writeln!(f)?;
        writeln!(f)?;

        Ok(())
    }

    fn write_lex_error(
        &self,
        f: &mut std::fmt::Formatter,
        details: &lex::LexerErrorDetails,
    ) -> std::fmt::Result {
        match &details {
            lex::LexerErrorDetails::InternalError(_) => {
                writeln!(f, "Internal error.")?;
            }
            lex::LexerErrorDetails::NothingMatched => {
                write!(f, "Encountered invalid characters. Previous tokens: ")?;
            }
        };

        Ok(())
    }

    fn write_parse_error(
        &self,
        f: &mut std::fmt::Formatter,
        tokens: &Vec<lex::DecoratedToken>,
        error: &ParserError,
    ) -> std::fmt::Result {
        let location = tokens
            .get(error.location)
            .map(|t| t.location())
            .unwrap_or(self.context.content().len());

        match &error.details {
            ParserErrorDetails::BranchingError(msg, errors) => {
                self.append_error(f, location, "Could not parse the remaining tokens.")?;
                for e in errors {
                    self.write_parse_error(f, tokens, e)?;
                }
            }
            ParserErrorDetails::ExpectedSomethingElse(a, b) => {
                self.append_error(
                    f,
                    location,
                    &format!("Expected {a}, but found {}", b.name()),
                )?;
            }
            ParserErrorDetails::Nested(msg, nested_error) => {
                self.append_error(f, location, msg)?;
                self.write_parse_error(f, tokens, nested_error)?;
            }
            e => {
                self.append_error(f, location, &format!("TODO: {e:#?}"))?;
            }
        };

        Ok(())
    }

    fn write_error(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "LexParseError:")?;

        match &self.details {
            LexParseErrorDetails::Lex { details } => {
                self.write_lex_error(f, &details.details)?;
            }
            LexParseErrorDetails::Parse { tokens, details } => {
                self.write_parse_error(f, tokens, details)?;
            }
            LexParseErrorDetails::Other(x) => writeln!(f, "  TODO: {x}")?,
        };
        Ok(())
    }
}

impl std::fmt::Display for LexParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_error(f)
    }
}
