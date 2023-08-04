use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub message: String
}

pub struct Context {
    pub stream: HashMap<String, Message>
}