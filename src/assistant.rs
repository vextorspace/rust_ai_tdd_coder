pub struct Assistant {

}

impl Assistant {
    pub fn new() -> Assistant {
        Assistant {}
    }
}
#[cfg(test)]
mod tests {
    use mockall::mock;
    use super::*;
    use std::path::PathBuf;
    use crate::test_runner::test_results::TestResults;

    #[test]
    fn instatiates() {
        mock!{
            pub TestRunner {
                fn run_tests(&self, path: PathBuf) -> TestResults;
            }
        }
        let _ = super::Assistant::new();

    }
}