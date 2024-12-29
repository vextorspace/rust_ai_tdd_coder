use anyhow::Result;
use crate::assistant::assistant::Assistant;

pub trait Command {
    fn should_execute(&self, command: &str) -> bool {
        self.get_label() == command
    }
    fn execute(&self, assistant: Box<&dyn Assistant>) -> Result<()>;
    fn get_label(&self) -> &str;
}