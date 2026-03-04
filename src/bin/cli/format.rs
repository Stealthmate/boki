use boki::lex;
use boki::lex::DecoratedToken;
use boki::parsing::TokenScanner;
use boki::tokens;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

mod _ast;
mod error;
mod parse;
mod write;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn read_file(filename: Rc<PathBuf>) -> Result<Rc<str>> {
    std::fs::read_to_string(filename.as_ref())
        .map_err(error::map_io_error(filename.clone()))
        .map(|x| x.into())
}

fn format_content(filename: Rc<PathBuf>, content: Rc<str>) -> Result<String> {
    let decorated_tokens: Rc<[DecoratedToken]> = lex::lex_string(content.as_ref())
        .map_err(error::map_lexer_error(filename.clone(), content.clone()))?
        .into();

    let tokens: Vec<tokens::Token> = decorated_tokens
        .iter()
        .map(|dt| dt.token().clone())
        .collect();

    let nodes = parse::parse(&mut TokenScanner::from_slice(tokens.as_slice())).map_err(
        error::map_parser_error(filename.clone(), content.clone(), decorated_tokens),
    )?;

    let output = format!("{}", write::to_displayable(nodes.as_slice()));

    Ok(output)
}

fn write_file(filename: Rc<PathBuf>, output: &str) -> Result<()> {
    let mut f = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open(filename.as_ref())
        .map_err(error::map_io_error(filename.clone()))?;
    f.write_all(output.as_bytes())
        .map_err(error::map_io_error(filename.clone()))?;

    Ok(())
}

fn format_file(filename: &Path) -> Result<()> {
    let filename = Rc::new(filename.to_path_buf());
    let content = read_file(filename.clone())?;
    let output = format_content(filename.clone(), content.clone())?;
    write_file(filename, &output)?;
    Ok(())
}

#[derive(clap::Args)]
pub struct Args {
    files: Vec<PathBuf>,
}

pub fn run(args: &Args) -> Result<()> {
    for file in &args.files {
        format_file(file)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::rc::Rc;

    use crate::{cli::format::read_file, error::CLIErrorResult};

    #[test]
    fn test_smoke() {
        let filename = Rc::new(PathBuf::from("src/bin/cli/format/input.boki"));
        let content = read_file(filename.clone()).or_panic();
        let formatted_str = super::format_content(filename.clone(), content.clone()).or_panic();
        let rhs = std::fs::read_to_string("src/bin/cli/format/output.boki")
            .expect("Could not read output file.");

        assert_eq!(formatted_str, rhs);
    }
}
