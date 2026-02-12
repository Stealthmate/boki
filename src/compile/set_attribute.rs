use super::CompilationResult;
use crate::contracts::output;

pub struct SetAttributeCompiler;

impl SetAttributeCompiler {
    pub fn compile(
        name: &str,
        value: &str,
        journal: &mut output::Journal,
    ) -> CompilationResult<()> {
        if name == "default_commodity" {
            journal.header.default_commodity = value.to_string();
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::SetAttributeCompiler;
    use crate::contracts::output;

    #[test]
    fn test_set_default_commodity() {
        let mut journal = output::Journal::default();
        SetAttributeCompiler::compile("default_commodity", "JPY", &mut journal).expect("Failed.");

        assert_eq!(journal.header.default_commodity, "JPY");
    }
}
