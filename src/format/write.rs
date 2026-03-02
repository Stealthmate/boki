//! This module handles converting the formatter's AST into text that further be written to a file.
use crate::format::_ast;
use crate::tokens;

#[derive(Clone, Debug)]
pub struct FormatContext {
    account_column_width: usize,
    commodity_column_width: usize,
    amount_column_width: usize,
}

#[allow(clippy::derivable_impls)]
impl Default for FormatContext {
    fn default() -> Self {
        Self {
            account_column_width: 0,
            commodity_column_width: 0,
            amount_column_width: 0,
        }
    }
}

/// Type wrapper with a [std::fmt::Display] derivation. We implement the derivation on the wrapper,
/// so that we don't force any specific logic on the AST (e.g. if we want to have multiple different Display derivations in the future).
pub(super) struct ToText<T>(FormatContext, T);

impl<T> ToText<T> {
    pub fn new(context: FormatContext, v: T) -> Self {
        Self(context, v)
    }

    fn with_context<T1>(&self, v: T1) -> ToText<T1> {
        ToText(self.0.clone(), v)
    }
}

impl<'a, T> std::fmt::Display for ToText<&'a [T]>
where
    ToText<&'a T>: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.1 {
            write!(f, "{}", self.with_context(item))?;
        }

        Ok(())
    }
}

impl std::fmt::Display for ToText<&tokens::Token> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
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
                writeln!(f, "  ---")?;
                let mut s = serde_yaml::to_string(mapping).unwrap();
                s = s.replace("\n", "\n  ");
                write!(f, "  {}", s)?;
                write!(f, "---")?;
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

impl std::fmt::Display for ToText<&_ast::Posting> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.with_context(&tokens::Token::Indent))?;
        write!(
            f,
            "{: <width$} ",
            format!("{}", self.with_context(self.1.account.as_slice())),
            width = self.0.account_column_width
        )?;
        write!(f, "{}", self.with_context(&tokens::Token::PostingSeparator))?;
        write!(
            f,
            " {: <width$} ",
            format!("{}", self.1.commodity.clone().unwrap_or("".to_string())),
            width = self.0.commodity_column_width
        )?;
        write!(f, "{}", self.with_context(&tokens::Token::PostingSeparator))?;

        if self.1.amount.is_some() || self.1.comment.is_some() {
            write!(
                f,
                "{: >width$}",
                format!(
                    "{}",
                    self.1
                        .amount
                        .map(|x| x.to_string())
                        .unwrap_or("".to_string())
                ),
                width = self.0.amount_column_width + 1
            )?;
            if let Some(comment) = &self.1.comment {
                write!(f, " //{}", comment)?;
            }
        }

        write!(f, "{}", self.with_context(&tokens::Token::LineSeparator))?;

        Ok(())
    }
}

impl std::fmt::Display for ToText<&_ast::Node> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            _ast::Node::Misc(tokens) => {
                write!(f, "{}", self.with_context(fold_tokens(tokens).as_slice()))?;
            }
            _ast::Node::Posting(posting) => {
                write!(f, "{}", self.with_context(posting.as_ref()))?;
            }
        };

        Ok(())
    }
}

fn compute_format(nodes: &[_ast::Node]) -> FormatContext {
    let mut ctx = FormatContext::default();

    for node in nodes {
        #[allow(clippy::single_match)]
        match node {
            _ast::Node::Posting(posting) => {
                let acct_string =
                    format!("{}", ToText::new(ctx.clone(), posting.account.as_slice()));
                ctx.account_column_width =
                    std::cmp::max(ctx.account_column_width, acct_string.len());

                ctx.commodity_column_width = std::cmp::max(
                    ctx.commodity_column_width,
                    posting.commodity.clone().unwrap_or("".to_string()).len(),
                );
                ctx.amount_column_width = std::cmp::max(
                    ctx.amount_column_width,
                    posting
                        .amount
                        .map(|x| x.to_string())
                        .unwrap_or("".to_string())
                        .len(),
                );
            }
            _ => {}
        }
    }

    ctx
}

pub(super) fn to_displayable(nodes: &[_ast::Node]) -> impl std::fmt::Display + '_ {
    let ctx = compute_format(nodes);
    ToText::new(ctx, nodes)
}

#[cfg(test)]
mod test {}
