use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub message: String
}

pub struct Context {
    pub stream: HashMap<String, Message>
}

impl Context {
    pub fn publish_message (&mut self, message: Message) {
        self.stream.insert("foo".to_string(), message);
    }
}