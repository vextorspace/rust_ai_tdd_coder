use crate::ai::commit_generator::CommitGeneratorBuilder;
use super::defined_assistant::DefinedAssistant;
use crate::git::version_control::VersionControlBuilder;
use crate::test_runner::test_provider::TestProviderFactory;

pub struct AssistantFactory{}

impl AssistantFactory {
    pub fn default() -> DefinedAssistant {
        let test_provider = TestProviderFactory::default();
        let version_controller = VersionControlBuilder::default();
        let commit_generator = CommitGeneratorBuilder::default();

        let assistant = DefinedAssistant::new(test_provider, version_controller, commit_generator);
        assistant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_makes_assistant_with_default_dependencies() {
        let assistant = AssistantFactory::default();
        let test_provider = assistant.test_provider;
        let version_control = assistant.version_control;
        let commit_generator = assistant.commit_generator;

        assert!(test_provider.is::<crate::test_runner::cargo_test_provider::CargoTestProvider>());
        assert!(version_control.is::<crate::git::git_version_control::GitVersionControl>());
        assert!(commit_generator.is::<crate::ai::constant_commit_message::ConstantCommitMessage>());
    }
}