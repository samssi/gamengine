use glfw::Key;
use crate::entity::entity::{Entity3d, Shading, TRIANGLE};
use crate::graphics::opengl::create_shader_programs;
use crate::io::keyboard::{create_keymap, key_to_string, KeyActivity};
use crate::io::loader::{read_fragment_shaders_into_memory, read_vertex_shaders_into_memory};
use crate::os::window_manager::{init_window_manager, start_opengl_window_manager};
use crate::state::context::{EntityContext, GameContext, ShaderContext};

fn glfw_press_handler(game_context: &mut GameContext, key: Key) {
    let entity = &mut game_context.entity_context.entities[0];
    let key_as_string = key_to_string(key);

    let direction = key_as_string.and_then(|key| game_context.window_context.keymap.get(&key));
    println!("Direction: {:?} pressed", direction);

    match direction {
        Some(KeyActivity::LEFT) => {
            let x_position = entity.transform.position.x;
            entity.transform.position.x = x_position - 0.1;
        }
        Some(KeyActivity::RIGHT) => {
            let x_position = entity.transform.position.x;
            entity.transform.position.x = x_position + 0.1;
        }
        Some(KeyActivity::UP) => {
            let y_position = entity.transform.position.y;
            entity.transform.position.y = y_position + 0.1;
        }
        Some(KeyActivity::DOWN) => {
            let y_position = entity.transform.position.y;
            entity.transform.position.y = y_position - 0.1;
        }
        Some(KeyActivity::ROTATE_CLOCKWISE) => {
            let rotation = entity.transform.rotation.z;
            entity.transform.rotation.z = rotation + 0.1;
        }
        Some(KeyActivity::ROTATE_COUNTER_CLOCKWISE) => {
            let rotation = entity.transform.rotation.z;
            entity.transform.rotation.z = rotation - 0.1;
        }
        Some(KeyActivity::SCALE_ALL_UP) => {
            let scale_x = entity.transform.scale.x;
            let scale_y = entity.transform.scale.y;
            let scale_z = entity.transform.scale.z;

            entity.transform.scale.x = scale_x + 0.1;
            entity.transform.scale.y = scale_y + 0.1;
            entity.transform.scale.z = scale_z + 0.1;
        }
        Some(KeyActivity::SCALE_ALL_DOWN) => {
            let scale_x = entity.transform.scale.x;
            let scale_y = entity.transform.scale.y;
            let scale_z = entity.transform.scale.z;

            entity.transform.scale.x = scale_x - 0.1;
            entity.transform.scale.y = scale_y - 0.1;
            entity.transform.scale.z = scale_z - 0.1;
        }
        _ => {}
    }
}

fn game_render_event(game_context: &mut GameContext) {
    let mut rotation = &mut game_context.entity_context.entities[0].transform.rotation;
    rotation.z = rotation.z + 0.01;
}

pub fn start() {
    let keymap = create_keymap();
    let basic_shading = Shading{
        vertex_shader: String::from("basic.vert"),
        fragment_shader: String::from("basic.frag")};

    let (window_context, events) = init_window_manager(keymap);
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

    let game_context = GameContext {
        shader_context,
        window_context,
        entity_context
    };

    start_opengl_window_manager(
        game_context,
        events,
        game_render_event,
        glfw_press_handler
    )
}