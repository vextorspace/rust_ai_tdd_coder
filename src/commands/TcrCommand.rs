use crate::commands::command::Command;
use anyhow::Result;

pub struct TcrCommand {
}

impl TcrCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for TcrCommand {
    fn execute(&self) -> Result<()> {
        Ok(())
    }

    fn get_label(&self) -> &str {
        "tcr"
    }
}

#[cfg(test)]
mod tests {
    use crate::commands::command::Command;
    use super::*;

    #[test]
    fn command_name_is_tcr() {
        let command: Box<dyn Command> = Box::new(TcrCommand::new());
        assert_eq!(command.get_label(), "tcr");
    }
}