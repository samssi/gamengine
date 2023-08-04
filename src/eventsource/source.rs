use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub message: String
}

pub struct Context {
    pub source: HashMap<String, Vec<Message>>
}

impl Context {
    pub fn publish_message (&mut self, key: String, message: Message) {
        match self.source.get_mut(&key) {
            Some(messages) => { messages.push(message); },
            None => { self.source.insert(key, vec![message]); }
        }
    }

    pub fn pop_message (&mut self, key: String) -> Option<Message> {
        match self.source.get_mut(&key) {
            Some(messages) => messages.pop(),
            None => None
        }
    }
}
