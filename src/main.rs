mod eventsource;

use std::collections::HashMap;

use crate::eventsource::source::Message;
use crate::eventsource::source::Context;

fn main() {
    let mut context = Context { stream: HashMap::new() };
    context.stream.insert(
        "foo".to_string(), 
        Message { message: "message".to_string() } 
    );
    println!("hashmap: {:?}", context.stream)
}
