use crate::ai::ai_provider::AiProvider;
use anyhow::{anyhow, Result};
use dotenv::var;
use llm_chain::parameters;
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain::prompt;
use llm_chain_openai::chatgpt::Executor;

pub(crate) struct OpenAiProvider {
    api_key: Option<String>,
    model: String,
    temperature: f32,
    max_tokens: u32,
}

impl AiProvider for OpenAiProvider {
    fn initialize_env(&mut self) {
        self.api_key = var("OPENAI_API_KEY").ok();
    }

    fn execute_query(&self, prompt: String) -> Result<String> {
        let res = tokio::runtime::Runtime::new()?.block_on(async {

            let exec = Executor::new().map_err(|e| anyhow!(e))?;

            prompt!(prompt).run(&parameters!(
                "temperature" => format!("{:.2}", self.temperature),
                "model" => self.model.clone(),
                "max_tokens" => format!("{}", self.max_tokens),
            ), &exec).await.map_err(|e| anyhow!(e))
        })?;

        Ok(res.to_string())
    }
}

impl OpenAiProvider {
    pub fn new() -> Self {
        OpenAiProvider {
            api_key: None,
            model: "gpt-4o".to_string(),
            temperature: 0.0,
            max_tokens: 1024,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_env() {
        let mut open_ai_provider = OpenAiProvider::new();

        open_ai_provider.initialize_env();

        assert!(open_ai_provider.api_key.is_some());
        assert_ne!(open_ai_provider.api_key.unwrap(), "", "You need an OPENAI_API_KEY set in .env");

        assert_eq!(open_ai_provider.model, "gpt-4o");
        assert_eq!(open_ai_provider.temperature, 0.0);
        assert_eq!(open_ai_provider.max_tokens, 1024);
    }

    #[test]
    fn test_execute_query() {
        let mut open_ai_provider = OpenAiProvider::new();
        open_ai_provider.initialize_env();
        let result = open_ai_provider.execute_query("Return the phrase Hippo World!".to_string());

        println!("==== RESULT: {:?}", result);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Hippo World"));
    }
}