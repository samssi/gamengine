use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub message: String
}

pub struct Context {
    pub stream: HashMap<String, Vec<Message>>
}

impl Context {
    pub fn publish_message (&mut self, key: String, message: Message) {
        match self.stream.get_mut(&key) {
            Some(messages) => { messages.push(message); },
            None => { self.stream.insert(key, vec![message]); }
        }
    }
}
