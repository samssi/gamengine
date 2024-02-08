use std::time::{SystemTime, UNIX_EPOCH};
use glfw::{Context, WindowEvent, GlfwReceiver};
use crate::entity::entity::{Entity3d, TRIANGLE};

use crate::graphics::opengl::{gl_render, gl_init};
use crate::io::keyboard::{create_keymap, handle_keyboard_events};
use crate::io::loader::read_fragment_shaders_into_memory;
use crate::state::context::WindowManagerContext;

const SCREEN_WIDTH: u32 = 1800;
const SCREEN_HEIGHT: u32 = 1000;

fn process_events<'context>(context: &'context mut WindowManagerContext, receiver: &GlfwReceiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&receiver) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                handle_keyboard_events(context, key, action)
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

    // TODO: handle error if time goes backwards
    let mut previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let keymap = create_keymap();
    let mut test_entity = Entity3d::new(TRIANGLE.to_vec());
    // TODO: useless temp
    let mut shaders = read_fragment_shaders_into_memory();

    let mut context = WindowManagerContext::new(
        &mut window,
        &keymap,
        &mut test_entity,
        &mut shaders);

    while !context.window.should_close() {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let delta_time = current_time - previous_time;

        previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        gl_init();
        gl_render(&mut context, delta_time);

        context.window.swap_buffers();
        glfw.poll_events();

        process_events(&mut context, &events)
    }
}