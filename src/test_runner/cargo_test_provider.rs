pub struct CargoTestProvider {}

impl CargoTestProvider {
    pub fn new() -> CargoTestProvider {
        CargoTestProvider {}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiates() {
        let _provider = CargoTestProvider::new();
    }
}