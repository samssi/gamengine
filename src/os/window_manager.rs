use std::time::{SystemTime, UNIX_EPOCH};
use glfw::{Context, WindowEvent, GlfwReceiver};
use crate::entity::entity::{Entity3d, Shading, TRIANGLE};

use crate::graphics::opengl::{gl_render, gl_init, create_shader_programs, link_program};
use crate::io::keyboard::{create_keymap, handle_keyboard_events};
use crate::io::loader::{read_fragment_shaders_into_memory, read_vertex_shaders_into_memory};
use crate::state::context::{WindowContext, WindowProperties};
use crate::state::entity_context::{EntityContext, ShaderContext};

fn process_events<'context>(window_context: &'context mut WindowContext, entity_context: &mut EntityContext, receiver: &GlfwReceiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&receiver) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                handle_keyboard_events(window_context, &mut entity_context.entities[0], key, action)
            },
        _ => {},
        }
    }
}

pub fn start_window_manager() {
    println!("Starting window manager!");
    let window_properties = WindowProperties{
        width: 800,
        height: 600
    };

    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(window_properties.width, window_properties.height, "GamEngine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // TODO: handle error if time goes backwards
    let mut previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let keymap = create_keymap();
    let basic_shading = Shading{vertex_shader: String::from("basic.vert"),
        fragment_shader: String::from("basic.frag")};

    let vertex_shaders = create_shader_programs(read_vertex_shaders_into_memory(), gl::VERTEX_SHADER);
    let fragment_shaders = create_shader_programs(read_fragment_shaders_into_memory(), gl::FRAGMENT_SHADER);
    let shader_context = ShaderContext{fragment_shaders, vertex_shaders};


    let mut entities: Vec<Entity3d> = vec![Entity3d::with_default_transform(
        shader_context,
        TRIANGLE.to_vec(),
        basic_shading
    )];

    let mut entity_context = EntityContext{
        entities: &mut entities
    };

    let mut context = WindowContext::new(
        &mut window,
        &window_properties,
        &keymap
    );

    while !context.window.should_close() {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let delta_time = current_time - previous_time;

        previous_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

        gl_init(&mut context);
        gl_render(&entity_context, delta_time);

        context.window.swap_buffers();
        glfw.poll_events();

        process_events(&mut context, &mut entity_context, &events)
    }
}