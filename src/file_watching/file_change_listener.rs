pub trait FileChangeListener {
    fn on_file_change(&self, file_path: &str);
}