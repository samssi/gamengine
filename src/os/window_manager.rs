use glfw::Action;
use glfw::{Context, Key, WindowEvent, GlfwReceiver, PWindow};

use crate::graphics::opengl::{gl_render, gl_init};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn process_events(window: &mut PWindow, receiver: &GlfwReceiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&receiver) {
        println!("{:?}", event);
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            },
        _ => {},
        }
    }
}

pub fn start_window_manager() {
    println!("Starting window manager!");

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

        while !window.should_close() {
            gl_init();
            gl_render();

            window.swap_buffers();
            glfw.poll_events();

            process_events(&mut window, &events)
        }
}