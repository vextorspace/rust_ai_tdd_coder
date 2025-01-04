use crate::ai::ai_provider::AiProvider;

pub struct AiCommitGenerator {}

impl AiCommitGenerator {
    pub fn new(ai_provider: Box<dyn AiProvider>) -> AiCommitGenerator {
        AiCommitGenerator {}
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
}