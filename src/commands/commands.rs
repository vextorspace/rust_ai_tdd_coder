use std::path::PathBuf;
use crate::commands::command::Command;
use anyhow::Result;
use crate::assistant::assistant::Assistant;

pub struct Commands {
    pub(crate) commands: Vec<Box<dyn Command>>,
}

impl Commands {
    pub fn add(&mut self, command: Box<dyn Command>) {
       self.commands.append(&mut vec![command]);
    }

    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn execute(&self, command: &str, assistant: Box<&dyn Assistant>, path: PathBuf) -> Result<()> {
        self.commands.iter().filter(|com| com.as_ref().get_label() == command)
            .nth(0)
            .map(|com| com.execute(assistant, path))
            .unwrap_or_else(|| Err(anyhow::anyhow!("Command not found")))
    }
}

#[cfg(test)]
mod tests {
    use crate::assistant::assistant_factory::AssistantFactory;
    use super::*;

    struct FakeCommand {
        name: String,
    }

    impl FakeCommand {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
            }
        }
    }

    impl Command for FakeCommand {
        fn execute(&self, _assistant: Box<&dyn Assistant>, _path: PathBuf) -> Result<()> {
            Ok(())
        }

        fn get_label(&self) -> &str {
            self.name.as_str()
        }
    }

    #[test]
    fn command_not_in_list_error() {
        let mut commands = Commands::new();
        commands.add(Box::new(FakeCommand::new("fred")));
        let assistant = AssistantFactory::default();
        assert!(commands.execute("barney", Box::new(&assistant), PathBuf::from("fred")).is_err());
    }

    #[test]
    fn command_in_list_good() {
        let mut commands = Commands::new();
        commands.add(Box::new(FakeCommand::new("fred")));
        let assistant = AssistantFactory::default();
        assert!(commands.execute("fred", Box::new(&assistant), PathBuf::from("bob")).is_ok());
    }
}