use anyhow::Result;

trait AiProvider {
    fn initialize_env(&mut self);
    fn query(&self, prompt: String) -> Result<String>;
}