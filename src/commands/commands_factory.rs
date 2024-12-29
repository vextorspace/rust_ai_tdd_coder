use crate::commands::command::Command;
use crate::commands::commands::Commands;
use crate::commands::tcr_command::TcrCommand;

struct CommandsFactory {
}

impl CommandsFactory {
    pub fn default() -> Commands {
        let mut commands = Commands::new();
        commands.add(Box::new(TcrCommand::new()));
        commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_contains_tcr() {
        let commands = CommandsFactory::default();
        assert!(commands.commands.iter().any(|com| com.as_ref().get_label() == "tcr"));
    }
}