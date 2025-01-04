use std::path::PathBuf;
use crate::ai::ai_provider::AiProvider;
use crate::ai::commit_generator::CommitGenerator;
use anyhow::Result;

pub struct AiCommitGenerator {
    ai_provider: Box<dyn AiProvider>,
}

impl AiCommitGenerator {
    pub fn new(ai_provider: Box<dyn AiProvider>) -> AiCommitGenerator {
        AiCommitGenerator {
            ai_provider,
        }
    }

    pub(crate) fn create_query(&self, diff: String) -> String {
        format!("
                            You are a terse and efficient developer.
                            Each change should be on its own line.
                            Each change message should be 50 characters or less.
                            Try to keep each change message below 6 words if possible.
                            An added or removed file should be mentioned in the message.
                                the diff is: {}:

                            Write a non-generic commit message. An example would be: \"added ability to print out the bill\"", diff)

    }
}

impl CommitGenerator for AiCommitGenerator {
    fn generate_commit_message(&self, diff: String) -> Result<String> {
        let query = self.create_query(diff);

        self.ai_provider.execute_query(query).map(|msg| msg.clone().strip_prefix("Assistant:").unwrap_or(msg.clone().as_str()).trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::ai::ai_provider::MockAiProvider;
    use super::*;

    #[test]
    fn instantiates() {
        let ai_provider = MockAiProvider::new();

        let _commit_generator = AiCommitGenerator::new(Box::new(ai_provider));
    }

    #[test]
    fn query_uses_diff() {
        let ai_provider = MockAiProvider::new();

        let commit_generator = AiCommitGenerator::new(Box::new(ai_provider));
        let diff_string = "::DIFF HERE::".to_string();
        let query = commit_generator.create_query(diff_string.clone());

        assert!(query.contains(&diff_string));
    }

    #[test]
    fn asking_for_commit_causes_query() {
        let mut ai_provider = MockAiProvider::new();
        ai_provider.expect_execute_query().times(1).returning(|_| Ok("Commit message".to_string()));

        let commit_generator = AiCommitGenerator::new(Box::new(ai_provider));
        let commit_message = commit_generator.generate_commit_message("diff".to_string());

        assert_eq!(commit_message.unwrap(), "Commit message");
    }
}