pub struct Assistant {

}

impl Assistant {
    pub fn new() -> Assistant {
        Assistant {}
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn instatiates() {
        let _ = super::Assistant::new();
    }
}