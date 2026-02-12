use crate::compile::CompilationError;
use crate::lexparse::LexParseError;

use crate::utils::indent_string;

#[derive(Clone, Debug)]
pub(super) enum EvaluateErrorContext {
    File(String),
}

impl std::fmt::Display for EvaluateErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(fp) => {
                write!(f, "file {}", fp)?;
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub(super) enum EvaluateErrorDetails {
    LexParseError(LexParseError),
    CompileError(CompilationError),
}

impl std::fmt::Display for EvaluateErrorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LexParseError(e) => writeln!(f, "{e}")?,
            Self::CompileError(e) => writeln!(f, "{e:#?}")?,
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct EvaluateError {
    pub(super) context: EvaluateErrorContext,
    pub(super) details: EvaluateErrorDetails,
}

impl std::fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Evaluation error:")?;
        writeln!(f, "  in {}", self.context)?;
        writeln!(f, "{}", indent_string(&format!("{}", self.details)))?;

        Ok(())
    }
}
