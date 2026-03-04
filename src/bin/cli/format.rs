use boki::parsing::TokenScanner;
use boki::{lex, parsing};
use boki::{tokens, utils};
use std::io::Write;
use std::path::{Path, PathBuf};

mod _ast;
mod parse;
mod write;

#[derive(Debug)]
pub struct Error {
    context: FormatContext,
    error: String,
}

impl crate::error::CLIError for Error {
    fn format(&self) -> String {
        let mut s = String::new();
        let ctx = &self.context;
        s += &format!(
            "Error while formatting file: {}",
            ctx.filename.as_os_str().to_str().unwrap()
        );
        if let Some(location) = ctx.location {
            let (line, character) = boki::utils::get_position_in_content(
                ctx.content.clone().unwrap().as_ref(),
                location,
            );
            s += &format!(":{}:{}", line + 1, character + 1);
        }
        s += ":\n";

        if ctx.location.is_some() {
            s += &utils::pretty_print_location(
                ctx.content.clone().unwrap().as_ref(),
                ctx.location.unwrap(),
            );
            s += "\n";
        }

        s += &self.error;
        s
    }
}

#[derive(Clone, Debug)]
struct FormatContext {
    filename: std::sync::Arc<PathBuf>,
    content: Option<std::sync::Arc<str>>,
    location: Option<usize>,
}

impl FormatContext {
    fn new(filename: PathBuf) -> Self {
        Self {
            filename: std::sync::Arc::from(filename),
            content: None,
            location: None,
        }
    }

    fn map_display_error<E: std::fmt::Display>(&self, e: E) -> Error {
        Error {
            context: Self {
                filename: self.filename.clone(),
                content: self.content.clone(),
                location: None,
            },
            error: format!("{e}"),
        }
    }

    fn map_lexer_error(&self, e: lex::LexerError) -> Error {
        Error {
            context: Self {
                filename: self.filename.clone(),
                content: self.content.clone(),
                location: Some(e.location),
            },
            error: format!("{e}"),
        }
    }

    fn map_parser_error(&self, e: parsing::ParserError) -> Error {
        Error {
            context: Self {
                filename: self.filename.clone(),
                content: self.content.clone(),
                location: Some(e.location),
            },
            error: format!("{e:#?}"),
        }
    }
}

fn format_content(ctx: &mut FormatContext) -> Result<String, Error> {
    let decoated_tokens = lex::lex_string(ctx.content.clone().unwrap().as_ref())
        .map_err(|e| ctx.map_lexer_error(e))?;
    let tokens: Vec<tokens::Token> = decoated_tokens
        .iter()
        .map(|dt| dt.token().clone())
        .collect();
    let nodes = parse::parse(&mut TokenScanner::from_slice(tokens.as_slice()))
        .map_err(|e| ctx.map_parser_error(e))?;

    let output = format!("{}", write::to_displayable(nodes.as_slice()));

    Ok(output)
}

fn read_file(ctx: &mut FormatContext) -> Result<(), Error> {
    let input =
        std::fs::read_to_string(ctx.filename.as_ref()).map_err(|e| ctx.map_display_error(e))?;
    ctx.content = Some(std::sync::Arc::from(input));

    Ok(())
}

fn write_file(ctx: &mut FormatContext, output: &str) -> Result<(), Error> {
    let mut f = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open(ctx.filename.as_ref())
        .map_err(|e| ctx.map_display_error(e))?;
    f.write_all(output.as_bytes())
        .map_err(|e| ctx.map_display_error(e))?;

    Ok(())
}

fn format_file(filename: &Path) -> Result<(), Error> {
    let mut ctx = FormatContext::new(filename.to_path_buf());
    read_file(&mut ctx)?;
    let output = format_content(&mut ctx)?;
    write_file(&mut ctx, &output)?;
    Ok(())
}

#[derive(clap::Args)]
pub struct Args {
    files: Vec<PathBuf>,
}

pub fn run(args: &Args) -> Result<(), Error> {
    for file in &args.files {
        format_file(file)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{cli::format::read_file, error::CLIErrorResult};

    #[test]
    fn test_smoke() {
        let mut ctx = super::FormatContext::new(PathBuf::from("src/bin/cli/format/input.boki"));
        read_file(&mut ctx).or_panic();
        let formatted_str = super::format_content(&mut ctx).or_panic();
        let rhs = std::fs::read_to_string("src/bin/cli/format/output.boki")
            .expect("Could not read output file.");

        assert_eq!(formatted_str, rhs);
    }
}
