pub trait MessageOutput {
    fn output(&mut self, message: &str);
}
