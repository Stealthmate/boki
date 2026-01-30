#![allow(dead_code)]
#![allow(unused_variables)]

mod compile;
mod lex;
mod parse;

pub fn compile_string(input: &str) -> Result<crate::output::Journal, String> {
    let (_, tokens) = lex::lex_string(input).map_err(|e| e.to_string())?;
    let (_, ast) = parse::parse_tokens(&tokens)?;
    compile::compile(ast)
}
