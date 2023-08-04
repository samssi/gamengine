mod eventsource;

use std::collections::HashMap;

use crate::eventsource::source::Message;
use crate::eventsource::source::Context;

fn main() {
    let mut context = Context { source: HashMap::new() };
    context.publish_message(
        "foo".to_string(),
        Message { message: "message".to_string() } 
    );

    context.publish_message(
        "bar".to_string(),
        Message { message: "message".to_string() } 
    );

    context.publish_message(
        "foo".to_string(),
        Message { message: "message2".to_string() } 
    );
    
    println!("hashmap: {:?}", context.source);

    let message = context.pop_message("foo".to_string());
    match message {
        Some(message) => println!("{}", message.message),
        None => println!("none")
    }

    println!("hashmap: {:?}", context.source);
}
