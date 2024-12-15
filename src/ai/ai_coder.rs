use anyhow::Result;

#[cfg_attr(test, mockall::automock)]
pub trait AiCoder {
    fn write_new_code(&self, code: String, tests: String) -> Result<String>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn good_test() {
        assert_eq!(1, 1);
    }
}