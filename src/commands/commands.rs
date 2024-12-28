use crate::commands::command::Command;
use anyhow::Result;

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

    pub fn execute(&self, command: &str) -> Result<()> {
        Err(anyhow::anyhow!("Command not found: {}", command))
    }
}

#[cfg(test)]
mod tests {
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
        fn execute(&self) -> Result<()> {
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

        assert!(commands.execute("barney").is_err());
    }


}