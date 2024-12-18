use super::version_control::VersionControl;

pub struct GitVersionControl{

}

impl GitVersionControl {
    pub(crate) fn new() -> Self {
        Self{}
    }
}

impl VersionControl for GitVersionControl {
    fn commit(&self, _path: &std::path::PathBuf) {
        todo!()
    }

    fn reject(&self, _path: &std::path::PathBuf) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::git::version_control::VersionControl;
    use super::*;

    #[test]
    fn instantiate() {
        let _: Box<dyn VersionControl> = Box::new(GitVersionControl::new());
    }
}
