use crate::test_runner::test_provider::TestProvider;
use crate::git::version_control::VersionControl;
use crate::ai::ai_coder::AiCoder;

pub struct Assistant {
    test_provider: Option<Box<dyn TestProvider>>,
    version_control: Option<Box<dyn VersionControl>>,
    ai_coder: Option<Box<dyn AiCoder>>,
}

impl Assistant {
    pub fn new() -> Assistant {
        Assistant {
            test_provider: None,
            version_control: None,
            ai_coder: None,
        }
    }

    pub fn builder() -> AssistantBuilder {
        AssistantBuilder::new()
    }
    pub fn with_test_provider(&mut self, test_provider: Box<dyn TestProvider>) -> &mut Self {
        self.test_provider = Some(test_provider);
        self
    }

    pub fn with_version_control(&mut self, version_control: Box<dyn VersionControl>) -> &mut Self {
        self.version_control = Some(version_control);
        self
    }

    pub fn with_ai_coder(&mut self, ai_coder: Box<dyn AiCoder>) -> &mut Self {
        self.ai_coder = Some(ai_coder);
        self
    }
}

pub struct AssistantBuilder {
}

impl AssistantBuilder {
    fn new() -> AssistantBuilder {
        AssistantBuilder {
        }
    }


    pub fn build(&self) -> Assistant {
        Assistant {
            test_provider: None,
            version_control: None,
            ai_coder: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_runner::test_provider::MockTestProvider;
    use crate::git::version_control::MockVersionControl;
    use crate::ai::ai_coder::MockAiCoder;

    #[test]
    fn builder_creates_empty() {
        let assistant = Assistant::builder().build();
        assert!(assistant.test_provider.is_none());
        assert!(assistant.version_control.is_none());
        assert!(assistant.ai_coder.is_none());
    }
}