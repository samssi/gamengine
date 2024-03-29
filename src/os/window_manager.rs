use std::time::{SystemTime, UNIX_EPOCH};
use glfw::{Context, CursorMode, GlfwReceiver, Key, PWindow, Window, WindowEvent};
use glfw::CursorMode::Disabled;

use crate::graphics::opengl::{create_program, create_shader_programs, init_renderer, link_shaders, render};
use crate::io::keyboard::{handle_keyboard_events, KeyActivity};
use crate::state::context::{Cursor, EntityContext, Game, GameContext, ShaderContext, WindowContext, WindowProperties, WindowState};

fn process_events<T> (
    game_context: &mut GameContext<T>,
    events: &GlfwReceiver<(f64, WindowEvent)>,
    glfw_press_handler: fn(game_context: &mut GameContext<T>, key: Key),
    glfw_cursor_handler: fn(game_context: &mut GameContext<T>, f64, f64)
    ) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            WindowEvent::CursorPos(x_pos, y_pos) => {
                glfw_cursor_handler(game_context, x_pos, y_pos)
            },
            WindowEvent::Key(key, _, action, _) => {
                handle_keyboard_events(game_context, key, action, glfw_press_handler)
            }
        _ => {},
        }
    }
}

fn process_cursor<T>(
    game_context: &mut GameContext<T>) {
    if game_context.window_context.window.is_hovered() {
        let (x_pos, y_pos) = game_context.window_context.window.get_cursor_pos();
        println!("{}, {}", x_pos, y_pos);
    }
}


pub fn init_opengl_window_manager() -> (WindowContext, GlfwReceiver<(f64, WindowEvent)>) {
    let window_properties = WindowProperties{
        width: 1600,
        height: 800
    };

    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events): (PWindow, GlfwReceiver<(f64, WindowEvent)>) =
        glfw.create_window(window_properties.width, window_properties.height, "GamEngine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(Disabled);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (cursor_x_pos, cursor_y_pos) = window.get_cursor_pos();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let window_state = WindowState{
        delta_time: current_time,
        cursor: Cursor{
            previous_x_pos: cursor_x_pos,
            previous_y_pos: cursor_y_pos,
        }
    };

    (WindowContext{
        window,
        window_state,
        window_properties,
        glfw
    }, events)
}

pub fn start_opengl_window_manager<'a, T>(
    game_context: &mut GameContext<T>,
    events: GlfwReceiver<(f64, WindowEvent)>,
    game_render_event: fn(game_context: &mut GameContext<T>),
    glfw_press_handler: fn(game_context: &mut GameContext<T>, key: Key),
    glfw_cursor_handler: fn(game_context: &mut GameContext<T>, f64, f64)) {
    println!("Starting window manager!");

    // TODO: handle error if time goes backwards
    let mut previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    while !game_context.window_context.window.should_close() {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        game_context.window_context.window_state.delta_time = current_time - previous_time;

        previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        init_renderer(&mut game_context.window_context);
        render(game_context);
        game_render_event(game_context);

        game_context.window_context.window.swap_buffers();
        game_context.window_context.glfw.poll_events();

        // process_cursor(&mut game_context);
        process_events(game_context, &events, glfw_press_handler, glfw_cursor_handler)
    }
}