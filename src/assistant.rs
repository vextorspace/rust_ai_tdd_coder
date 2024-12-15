use crate::test_runner::test_provider::TestProvider;
use crate::git::version_control::VersionControl;
use crate::ai::ai_coder::AiCoder;
use std::path::PathBuf;

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

    pub fn tcr(&self, path: PathBuf) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_runner::test_provider::MockTestProvider;
    use crate::git::version_control::MockVersionControl;
    use crate::ai::ai_coder::MockAiCoder;

    #[test]
    fn instantiates() {
        let test_provider = Box::new(MockTestProvider::new());
        let version_control = Box::new(MockVersionControl::new());
        let ai_coder = Box::new(MockAiCoder::new());
        let _assistant = Assistant::new(test_provider, version_control, ai_coder);
    }

}