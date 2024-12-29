use std::path::PathBuf;
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
    fn execute(&self, assistant: Box<&dyn Assistant>, path: PathBuf) -> Result<()> {
        assistant.tcr(path)
    }

    fn get_label(&self) -> &str {
        "tcr"
    }
}

#[cfg(test)]
mod tests {
    use crate::assistant::assistant::MockAssistant;
    use super::*;
    use crate::commands::command::Command;

    #[test]
    fn command_name_is_tcr() {
        let command: Box<dyn Command> = Box::new(TcrCommand::new());
        assert_eq!(command.get_label(), "tcr");
    }

    #[test]
    fn passing_tests_lead_to_message_and_commit() {
        let mut assistant = MockAssistant::new();
        assistant.expect_tcr().times(1).return_once(|_| Ok(()));
        let command = TcrCommand::new();
        command.execute(Box::new(&assistant), PathBuf::from("bob")).expect("should not fail");
    }
}