use anyhow::Result;

#[cfg_attr(test, mockall::automock)]
pub trait AiProvider {
    fn initialize_env(&mut self);
    fn execute_query(&self, prompt: String) -> Result<String>;
}