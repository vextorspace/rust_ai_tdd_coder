use anyhow::Result;

pub trait Command {
    fn should_execute(&self, command: &str) -> bool {
        self.get_label() == command
    }
    fn execute(&self) -> Result<()>;
    fn get_label(&self) -> &str;
}