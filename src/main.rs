mod eventsource;
mod graphics;
mod os;

use dashmap::DashMap;

use crate::eventsource::source::Message;
use crate::eventsource::source::GEContext;
use crate::os::window_manager::start_window_manager;

#[allow(dead_code)]
fn update() {
    let mut context = GEContext { source: DashMap::new() };
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

fn main() {
    update();
    start_window_manager();
}