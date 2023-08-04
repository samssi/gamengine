mod eventsource;

use std::collections::HashMap;

use crate::eventsource::source::Message;
use crate::eventsource::source::Context;

fn main() {
    let mut context = Context { stream: HashMap::new() };
    context.publish_message(
        Message { message: "message".to_string() } 
    );
    println!("hashmap: {:?}", context.stream);
}
