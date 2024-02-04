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

    pub fn remove_message (&mut self, key: String) {
        if self.source.contains_key(&key) { self.source.remove(&key); }
    }

    pub fn pop_message (&mut self, key: String) -> Option<Message> {
        let optional_message = match self.source.get(&key) {
            Some(message) => {
                let value = message.value().clone();
                Some(value)
            }
            None => None
        };
        
        self.remove_message(key);

        return optional_message
    }
}
