use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct Message {
    pub message: String
}

pub struct GEContext {
    pub source: DashMap<String, Message>
}

impl GEContext {
    pub fn publish_message (&mut self, key: String, message: Message) {
        self.source.insert(key, message);
    }

    pub fn get_message (&mut self, key: String) -> Option<Message> {
        match self.source.get(&key) {
            Some(message) => Some(message.value().clone()),
            None => None
        }
    }
}
