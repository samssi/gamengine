use crate::entity::entity::{Entity3d, Shading, TRIANGLE};
use crate::graphics::opengl::create_shader_programs;
use crate::io::keyboard::create_keymap;
use crate::io::loader::{read_fragment_shaders_into_memory, read_vertex_shaders_into_memory};
use crate::os::window_manager::{init_window_manager, start_window_manager};
use crate::state::entity_context::{EntityContext, ShaderContext};

pub fn start() {
    let keymap = create_keymap();
    let basic_shading = Shading{
        vertex_shader: String::from("basic.vert"),
        fragment_shader: String::from("basic.frag")};

    let (mut window_context, events) = init_window_manager(keymap);
    let vertex_shaders = create_shader_programs(read_vertex_shaders_into_memory(), gl::VERTEX_SHADER);
    let fragment_shaders = create_shader_programs(read_fragment_shaders_into_memory(), gl::FRAGMENT_SHADER);
    let shader_context = ShaderContext{fragment_shaders, vertex_shaders};


    let mut entities: Vec<Entity3d> = vec![Entity3d::with_default_transform(
        shader_context,
        TRIANGLE.to_vec(),
        basic_shading
    )];

    let mut entity_context = EntityContext{
        entities
    };

    start_window_manager(&mut window_context, &mut entity_context, events)
}