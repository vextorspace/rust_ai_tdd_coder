use crate::commands::command::Command;
use anyhow::Result;
use crate::assistant::assistant::Assistant;

pub struct TcrCommand {
}

impl TcrCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for TcrCommand {
    fn execute(&self, assistant: Box<&dyn Assistant>) -> Result<()> {
        Ok(())
    }

    fn get_label(&self) -> &str {
        "tcr"
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use crate::commands::command::Command;

    struct MockAssistant {
    }

    impl MockAssistant {
        fn new() -> Self {
            Self {}
        }
    }

    impl Assistant for MockAssistant {
        fn tcr(&self, _path: PathBuf) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn command_name_is_tcr() {
        let command: Box<dyn Command> = Box::new(TcrCommand::new());
        assert_eq!(command.get_label(), "tcr");
    }

    #[test]
    fn passing_tests_lead_to_message_and_commit() {
        let assistant = MockAssistant::new();
        let command = TcrCommand::new();
        command.execute(Box::new(&assistant)).expect("should not fail");

    }
}