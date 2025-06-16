use crate::traits::message_output::MessageOutput;

pub struct CliOutput;

impl MessageOutput for CliOutput {
    fn output(&mut self, message: &str) {
        println!("{}", message);
    }
}
