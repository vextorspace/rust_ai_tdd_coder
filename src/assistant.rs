use crate::test_runner::test_provider::TestProvider;
use crate::git::version_control::VersionControl;

pub struct Assistant {
    test_provider: Option<Box<dyn TestProvider>>,
    version_control: Option<Box<dyn VersionControl>>
}

impl Assistant {
    pub fn new() -> Assistant {
        Assistant {
            test_provider: None,
            version_control: None,
        }
    }

    pub fn with_test_provider(&mut self, test_provider: Box<dyn TestProvider>) -> &mut Self {
        self.test_provider = Some(test_provider);
        self
    }

    pub fn with_version_control(&mut self, version_control: Box<dyn VersionControl>) -> &mut Self {
        self.version_control = Some(version_control);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_runner::test_provider::MockTestProvider;
    use crate::git::version_control::MockVersionControl;
    use crate::ai::ai_coder::MockAiCoder;

    #[test]
    fn instatiates() {
        let mut mock_test_provider = MockTestProvider::new();
        mock_test_provider
            .expect_run_tests()
            .times(0);

        let mut mock_version_control = MockVersionControl::new();
        mock_version_control
            .expect_commit()
            .times(0);
        mock_version_control
            .expect_reject()
            .times(0);

        let mut mock_ai_coder = MockAiCoder::new();

        let _ = Assistant::new()
            .with_test_provider(
                Box::new(mock_test_provider)
            ).with_version_control(
                Box::new(mock_version_control)
        );
    }
}