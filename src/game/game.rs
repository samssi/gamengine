use glfw::Key;
use crate::entity::entity::{Entity3d, Shading, TRIANGLE};
use crate::game::keyboard_handler::glfw_press_handler;
use crate::graphics::opengl::create_shader_programs;
use crate::io::keyboard::{create_keymap};
use crate::io::loader::{read_fragment_shaders_into_memory, read_vertex_shaders_into_memory};
use crate::os::window_manager::{init_window_manager, start_opengl_window_manager};
use crate::state::context::{EntityContext, GameContext, KeyboardContext, ShaderContext};



fn game_render_event(game_context: &mut GameContext) {
    let mut rotation = &mut game_context.entity_context.entities[0].transform.rotation;
    rotation.z = rotation.z + 0.01;
}

pub fn start() {
    let keymap = create_keymap();
    let basic_shading = Shading{
        vertex_shader: String::from("basic.vert"),
        fragment_shader: String::from("basic.frag")};

    let (window_context, events) = init_window_manager();
    let vertex_shaders = create_shader_programs(read_vertex_shaders_into_memory(), gl::VERTEX_SHADER);
    let fragment_shaders = create_shader_programs(read_fragment_shaders_into_memory(), gl::FRAGMENT_SHADER);
    let shader_context = ShaderContext{fragment_shaders, vertex_shaders};

    let mut entities: Vec<Entity3d> = vec![Entity3d::with_default_transform(
        &shader_context,
        TRIANGLE.to_vec(),
        basic_shading
    )];

    let entity_context = EntityContext{
        entities
    };

    let keyboard_context = KeyboardContext{
        keymap
    };

    let game_context = GameContext {
        shader_context,
        window_context,
        entity_context,
        keyboard_context
    };

    start_opengl_window_manager(
        game_context,
        events,
        game_render_event,
        glfw_press_handler
    )
}