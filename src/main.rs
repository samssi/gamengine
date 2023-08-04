mod eventsource;
mod graphics;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::eventsource::source::Message;
use crate::eventsource::source::Context;
use crate::graphics::opengl::render;

#[allow(dead_code)]
fn update() {
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
fn print(count: i32) {
    println!("Running... Count {}", count)
}

/*fn main() {
    // update();
    let mut count = 0;
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    loop {
        count += 1;
        print(count);
        render();
        if !running.load(Ordering::SeqCst) {
            println!("Program was terminated");
            break;
        }
       
    }
}*/

fn main() {
    render();
}