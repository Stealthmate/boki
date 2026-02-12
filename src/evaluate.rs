#![allow(dead_code)]
#![allow(unused_variables)]

use crate::compile;
use crate::contracts::output;
use crate::lexparse;

#[derive(Clone, Debug)]
enum EvaluateErrorContext {
    File(String),
}

#[derive(Debug)]
enum EvaluateErrorDetails {
    LexParseError(lexparse::LexParseError),
    CompileError(compile::CompilationError),
}

#[derive(Debug)]
pub struct EvaluateError {
    context: EvaluateErrorContext,
    details: EvaluateErrorDetails,
}

impl std::fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "TODO")
    }
}

pub type EvaluateResult<T> = Result<T, Box<EvaluateError>>;

pub fn evaluate_file(filename: &str) -> EvaluateResult<output::Journal> {
    let context = EvaluateErrorContext::File(filename.to_string());

    let nodes = lexparse::lexparse_file(filename).map_err(|e| EvaluateError {
        context: context.clone(),
        details: EvaluateErrorDetails::LexParseError(*e),
    })?;
    let mut journal = output::Journal::default();
    for node in nodes {
        compile::compile_node(&node, &mut journal).map_err(|e| EvaluateError {
            context: context.clone(),
            details: EvaluateErrorDetails::CompileError(e),
        })?;
    }

    Ok(journal)
}
