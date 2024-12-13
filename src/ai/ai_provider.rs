use anyhow::Result;

trait AiProvider {
    fn initialize_env(&mut self);
}