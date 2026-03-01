//! This module handles converting the formatter's AST into text that further be written to a file.
use crate::format::_ast;
use crate::tokens;

/// Type wrapper with a [std::fmt::Display] derivation. We implement the derivation on the wrapper,
/// so that we don't force any specific logic on the AST (e.g. if we want to have multiple different Display derivations in the future).
pub(super) struct ToText<T>(T);

impl<'a> From<&'a tokens::Token> for ToText<&'a tokens::Token> {
    fn from(value: &'a tokens::Token) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a _ast::Node> for ToText<&'a _ast::Node> {
    fn from(value: &'a _ast::Node) -> Self {
        Self(value)
    }
}

impl<'a, T> std::fmt::Display for ToText<&'a [T]>
where
    ToText<&'a T>: From<&'a T> + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.0 {
            write!(f, "{}", ToText::from(item))?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ToText<&tokens::Token> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            tokens::Token::Keyword(kw) => write!(
                f,
                "{}",
                match kw {
                    tokens::Keyword::Set => "set",
                }
            ),
            tokens::Token::Timestamp(ts) => {
                if ts.time() == chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap() {
                    write!(f, "{}", ts.date_naive())?;
                }

                Ok(())
            }
            tokens::Token::Amount(amt) => write!(f, "{}", amt),
            tokens::Token::YamlMatter(mapping) => {
                write!(f, "  ")?;
                let mut s = serde_yaml::to_string(mapping).unwrap();
                s = s.replace("\n", "\n  ");
                write!(f, "{}", s)?;
                Ok(())
            }
            tokens::Token::LineSeparator => writeln!(f),
            tokens::Token::AccountSeparator => write!(f, "/"),
            tokens::Token::PostingSeparator => write!(f, ";"),
            tokens::Token::Identifier(x) => write!(f, "{}", x),
            tokens::Token::Comment(x) => write!(f, "//{}", x),
            tokens::Token::Whitespace => write!(f, " "),
            tokens::Token::Indent => write!(f, "  "),
            tokens::Token::Eof => Ok(()),
        }
    }
}

fn fold_tokens(tokens: &[tokens::Token]) -> Vec<tokens::Token> {
    tokens.iter().fold(vec![], |mut a, t| {
        match (a.last(), t) {
            // Trim consecutive whitespace
            (Some(tokens::Token::Whitespace), tokens::Token::Whitespace) => a,
            // If whitespace is followed by line separator, we ignore the whitespace
            (Some(tokens::Token::Whitespace), tokens::Token::LineSeparator) => {
                a.pop();
                a.push(t.clone());
                a
            }
            // Otherwise we do nothing
            _ => {
                a.push(t.clone());
                a
            }
        }
    })
}

impl std::fmt::Display for ToText<&_ast::Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            _ast::Node::Misc(tokens) => {
                write!(f, "{}", ToText(fold_tokens(tokens).as_slice()))?;
            }
        };

        Ok(())
    }
}

pub(super) fn to_displayable(nodes: &[_ast::Node]) -> impl std::fmt::Display + '_ {
    ToText(nodes)
}

#[cfg(test)]
mod test {}
