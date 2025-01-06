use std::path::PathBuf;
use crate::assistant::assistant::Assistant;
use crate::commands::command::Command;
pub(crate) struct WatchCommand {
}

impl WatchCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for WatchCommand {
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

    #[test]
    fn watch_tcr_is_name() {
        let command: Box<dyn Command> = Box::new(WatchCommand::new());

        assert_eq!(command.as_ref().get_label(), "watch_tcr");
    }


}