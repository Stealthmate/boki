use std::path::PathBuf;
use std::rc::Rc;

use boki::{compile, lex, output};

mod error;
mod parse;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn read_file(filename: Rc<PathBuf>) -> Result<Rc<str>> {
    std::fs::read_to_string(filename.as_ref())
        .map_err(error::map_io_error(filename.clone()))
        .map(|x| x.into())
}

fn compile_file(filename: Rc<PathBuf>) -> Result<output::Journal> {
    let content = read_file(filename.clone())?;

    let decorated_tokens: Rc<[lex::DecoratedToken]> = lex::lex_string(content.as_ref())
        .map_err(error::map_lexer_error(filename.clone(), content.clone()))?
        .into();
    let nodes = parse::parse_tokens(decorated_tokens.clone()).map_err(error::map_parser_error(
        filename.clone(),
        content.clone(),
        decorated_tokens.clone(),
    ))?;

    let mut journal = output::Journal::default();
    for node in nodes {
        println!("Node:\n{node:#?}");
        compile::compile_node(&node, &mut journal).map_err(error::map_compile_error())?;
    }

    Ok(journal)
}

#[derive(clap::Args)]
pub struct Args {
    file: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

pub fn run(args: &Args) -> Result<()> {
    let filename = Rc::new(args.file.clone());
    let journal = compile_file(filename.clone())?;
    let output_str = serde_json::to_string(&journal).map_err(error::map_serde_error())?;

    match &args.output {
        None => println!("{output_str}"),
        Some(x) => std::fs::write(x, output_str).expect("Failed to write output file."),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::rc::Rc;

    #[test]
    fn test_smoke() {
        let filename = Rc::new(PathBuf::from("docs/examples/01-books-journal/books.boki"));
        super::compile_file(filename.clone()).expect("Failed.");
    }
}
