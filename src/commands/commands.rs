use crate::commands::command::Command;
use anyhow::Result;
use crate::assistant::assistant::Assistant;

pub struct Commands {
    commands: Vec<Box<dyn Command>>,
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

    pub fn execute(&self, command: &str, assistant: &Assistant) -> Result<()> {
        self.commands.iter().filter(|com| com.as_ref().get_label() == command)
            .nth(0)
            .map(|com| com.execute(assistant))
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
        fn execute(&self, assistant: &Assistant) -> Result<()> {
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
        assert!(commands.execute("barney", &assistant).is_err());
    }

    #[test]
    fn command_in_list_good() {
        let mut commands = Commands::new();
        commands.add(Box::new(FakeCommand::new("fred")));
        let assistant = AssistantFactory::default();
        assert!(commands.execute("fred", &assistant).is_ok());
    }
}