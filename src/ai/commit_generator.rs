use crate::ai::constant_commit_message::ConstantCommitMessage;
use anyhow::Result;
use downcast_rs::{impl_downcast, Downcast};

#[cfg_attr(test, mockall::automock)]
pub trait CommitGenerator: Downcast {
    fn generate_commit_message(&self, diff: String) -> Result<String>;
}

impl_downcast!(CommitGenerator);

pub struct CommitGeneratorBuilder{}

impl CommitGeneratorBuilder {
    pub fn default() -> Box<dyn CommitGenerator> {
        Box::new(ConstantCommitMessage::new("Working...".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_makes_constant_commit_message() {
        let commit_generator = CommitGeneratorBuilder::default();
        assert!(commit_generator.as_ref().downcast_ref::<ConstantCommitMessage>().is_some())
    }
}