use glfw::Action;
use glfw::{Context, Key};

mod eventsource;
mod graphics;

use std::collections::HashMap;

use crate::eventsource::source::Message;
use crate::eventsource::source::GEContext;

#[allow(dead_code)]
fn update() {
    let mut context = GEContext { source: HashMap::new() };
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

fn report_run_count(count: i32) {
    println!("Running... Count {}", count)
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "GamEngine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mut count = 0;

        while !window.should_close() {
            count += 1;
            report_run_count(count);
            window.swap_buffers();
            glfw.poll_events();

            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    },
                _ => {},
            }
        }
    }
}