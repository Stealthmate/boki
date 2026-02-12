#![allow(dead_code)]
#![allow(unused_variables)]

use crate::compile;
use crate::contracts::output;
use crate::lexparse;

mod error;

pub use error::EvaluateError;
use error::{EvaluateErrorContext, EvaluateErrorDetails};

pub type EvaluateResult<T> = Result<T, Box<error::EvaluateError>>;

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
