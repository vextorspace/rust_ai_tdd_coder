use crate::test_runner::test_provider::TestProvider;
use crate::git::version_control::VersionControl;
use crate::ai::ai_coder::AiCoder;
use std::path::PathBuf;
use crate::test_runner::test_results::TestResults;

pub struct Assistant {
    test_provider: Box<dyn TestProvider>,
    version_control: Box<dyn VersionControl>,
    ai_coder: Box<dyn AiCoder>,
}

impl Assistant {
    pub fn new(
        test_provider: Box<dyn TestProvider>,
        version_control: Box<dyn VersionControl>,
        ai_coder: Box<dyn AiCoder>,
    ) -> Assistant {
        Assistant {
            test_provider: test_provider,
            version_control: version_control,
            ai_coder: ai_coder,
        }
    }

    pub fn tcr(&self, path: &PathBuf) {
        let results = self.test_provider.run_tests(&path);

        match results {
            TestResults::PASSED => {
                self.version_control.commit(&path);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_runner::test_provider::MockTestProvider;
    use crate::git::version_control::MockVersionControl;
    use crate::ai::ai_coder::MockAiCoder;
    use crate::test_runner::test_results::TestResults;

    #[test]
    fn instantiates() {
        let test_provider = Box::new(MockTestProvider::new());
        let version_control = Box::new(MockVersionControl::new());
        let ai_coder = Box::new(MockAiCoder::new());
        let _assistant = Assistant::new(test_provider, version_control, ai_coder);
    }

    #[test]
    fn tcr_calls_tests() {
        let mut test_provider = MockTestProvider::new();
        test_provider.expect_run_tests().times(1).return_const(TestResults::PASSED);
        let mut control = MockVersionControl::new();
        control.expect_commit().return_const(());
        let version_control = Box::new(control);
        let ai_coder = Box::new(MockAiCoder::new());
        let assistant = Assistant::new(Box::new(test_provider), version_control, ai_coder);
        assistant.tcr(&PathBuf::new());
    }

    #[test]
    fn tcr_commits_if_tests_passed() {
        let mut test_provider = MockTestProvider::new();
        test_provider.expect_run_tests().return_const(TestResults::PASSED);
        let mut version_control = MockVersionControl::new();
        version_control.expect_commit().times(1).return_const(());
        version_control.expect_reject().times(0).return_const(());
        let ai_coder = Box::new(MockAiCoder::new());
        let assistant = Assistant::new(Box::new(test_provider), Box::new(version_control), ai_coder);
        assistant.tcr(&PathBuf::new());
    }

}