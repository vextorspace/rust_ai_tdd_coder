use std::path::PathBuf;
use crate::ai::commit_generator::CommitGenerator;

pub struct ConstantCommitMessage {
    pub message: String,
}

impl ConstantCommitMessage {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl CommitGenerator for ConstantCommitMessage {
    fn generate_commit_message(&self, _path: &PathBuf, _diff: String) -> String {
        self.message.clone()
    }
}
