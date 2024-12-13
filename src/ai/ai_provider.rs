use anyhow::Result;

trait AiProvider {
    fn initialize_env(&mut self);
    fn execute_query(&self, prompt: String) -> Result<String>;
}