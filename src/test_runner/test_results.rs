#[derive(Debug, Clone)]
pub enum TestResults {
    PASSED,
    FAILED(String),
}


#[cfg(test)]
mod tests {
    #[test]
    fn can_clone() {
        let result = super::TestResults::PASSED;
        let _ = result.clone();
    }
}