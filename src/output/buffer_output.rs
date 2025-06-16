use crate::traits::message_output::MessageOutput;

pub struct BufferOutput {
    pub buffer: Vec<String>,
}

impl BufferOutput {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn into_messages(self) -> Vec<String> {
        self.buffer
    }
}

impl MessageOutput for BufferOutput {
    fn output(&mut self, message: &str) {
        self.buffer.push(message.to_string());
    }
}
