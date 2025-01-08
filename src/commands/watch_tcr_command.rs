use crate::assistant::assistant::Assistant;
use crate::commands::command::Command;
use std::path::PathBuf;
pub(crate) struct WatchTcrCommand {
}

impl WatchTcrCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for WatchTcrCommand {
    fn execute(&self, assistant: Box<&dyn Assistant>, path: PathBuf) -> anyhow::Result<()> {
        todo!()
    }

    fn get_label(&self) -> &str {
        "watch_tcr"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assistant::assistant::MockAssistant;

    #[test]
    fn watch_tcr_is_name() {
        let command: Box<dyn Command> = Box::new(WatchTcrCommand::new());

        assert_eq!(command.as_ref().get_label(), "watch_tcr");
    }
}