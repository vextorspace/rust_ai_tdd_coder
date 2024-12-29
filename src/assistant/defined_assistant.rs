use crate::test_runner::test_provider::TestProvider;
use crate::git::version_control::VersionControl;
use crate::ai::ai_coder::AiCoder;
use std::path::PathBuf;
use crate::test_runner::test_results::TestResults;
use anyhow::Result;
use crate::ai::commit_generator::CommitGenerator;
use crate::assistant::assistant::Assistant;

pub struct DefinedAssistant {
    pub(crate) test_provider: Box<dyn TestProvider>,
    pub(crate) version_control: Box<dyn VersionControl>,
    ai_coder: Option<Box<dyn AiCoder>>,
    pub(crate) commit_generator: Box<dyn CommitGenerator>,
}

impl DefinedAssistant {
    pub fn new(
        test_provider: Box<dyn TestProvider>,
        version_control: Box<dyn VersionControl>,
        commit_generator: Box<dyn CommitGenerator>,
    ) -> DefinedAssistant {
        DefinedAssistant {
            test_provider,
            version_control,
            ai_coder: None,
            commit_generator,
        }
    }
}

impl Assistant for DefinedAssistant {
    fn tcr(&self, path: PathBuf) -> Result<()> {
        let results = self.test_provider.run_tests(&path);

        match results {
            TestResults::PASSED => {
                let diff = self.version_control.diff(&path)?;
                let commit_message = self.commit_generator.generate_commit_message(&path, diff);
                self.version_control.commit(&path, commit_message)?;
            }
            TestResults::FAILED(_result) => {
                self.version_control.reject(&path)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_runner::test_provider::MockTestProvider;
    use crate::git::version_control::MockVersionControl;
    use crate::ai::commit_generator::MockCommitGenerator;
    use crate::test_runner::test_results::TestResults;

    #[test]
    fn instantiates() {
        let test_provider = Box::new(MockTestProvider::new());
        let version_control = Box::new(MockVersionControl::new());
        let commit_generator = Box::new(MockCommitGenerator::new());
        let _assistant = DefinedAssistant::new(test_provider, version_control, commit_generator);
    }

    #[test]
    fn tcr_calls_tests() {
        let mut test_provider = MockTestProvider::new();
        test_provider.expect_run_tests().times(1).return_const(TestResults::PASSED);
        let mut control = MockVersionControl::new();
        control.expect_commit().return_once(|_,_| Ok(()));
        control.expect_diff().returning(|_| Ok(String::from("diff +println('hippo');")));
        let version_control = Box::new(control);
        let mut generator = MockCommitGenerator::new();
        generator.expect_generate_commit_message().times(1).return_const("working".to_string());
        let commit_generator = Box::new(generator);
        let assistant = DefinedAssistant::new(Box::new(test_provider), version_control, commit_generator);
        assistant.tcr(PathBuf::new()).expect("should not fail");
    }

    #[test]
    fn tcr_commits_if_tests_passed() {
        let mut test_provider = MockTestProvider::new();
        test_provider.expect_run_tests().return_const(TestResults::PASSED);
        let mut version_control = MockVersionControl::new();
        version_control.expect_commit().times(1).return_once(|_,_| Ok(()));
        version_control.expect_reject().times(0);
        version_control.expect_diff().times(1).returning(|_| Ok(String::from("diff +println('hippo');")));
        let mut generator = MockCommitGenerator::new();
        generator.expect_generate_commit_message().times(1).return_const("working".to_string());
        let commit_generator = Box::new(generator);
        let assistant = DefinedAssistant::new(Box::new(test_provider), Box::new(version_control), commit_generator);
        assistant.tcr(PathBuf::new()).expect("should not fail");
    }

    #[test]
    fn tcr_rejects_if_tests_failed() {
        let mut test_provider = MockTestProvider::new();
        test_provider.expect_run_tests().return_const(TestResults::FAILED("".to_string()));
        let mut version_control = MockVersionControl::new();
        version_control.expect_commit().times(0);
        version_control.expect_reject().times(1).return_once(|_| Ok(()));
        version_control.expect_diff().times(0);
        let mut generator = MockCommitGenerator::new();
        generator.expect_generate_commit_message().times(0);
        let assistant = DefinedAssistant::new(Box::new(test_provider), Box::new(version_control), Box::new(generator));
        assistant.tcr(PathBuf::new()).expect("should not fail");
    }
}