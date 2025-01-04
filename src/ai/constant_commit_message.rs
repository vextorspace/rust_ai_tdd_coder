use crate::ai::commit_generator::CommitGenerator;
use anyhow::Result;


pub struct ConstantCommitMessage {
    pub message: String,
}

impl ConstantCommitMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl CommitGenerator for ConstantCommitMessage {
    fn generate_commit_message(&self, _diff: String) -> Result<String> {
        Ok(self.message.clone())
    }
}
