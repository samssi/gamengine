use std::collections::HashMap;
use glfw::{GlfwReceiver, Key, WindowEvent};
use crate::entity::camera::Camera;
use crate::entity::entity::{Entity3d};
use crate::entity::structures::{Transform, Vector3d};
use crate::game::context::GameState;
use crate::game::context::Mode::OBJECT;
use crate::game::keyboard_handler::glfw_press_handler;
use crate::game::window_handler::glfw_cursor_handler;
use crate::graphics::opengl::{create_program, create_shader_programs};
use crate::io::keyboard::{create_keymap};
use crate::io::loader::{read_fragment_shaders_into_memory, read_object_files_into_memory, read_vertex_shaders_into_memory};
use crate::io::object::{wavefront_object_as_points};
use crate::os::window_manager::{init_opengl_window_manager, start_opengl_window_manager};
use crate::state::context::{EntityContext, Game, GameContext, KeyboardContext, MouseContext, ObjectContext, ShaderContext};

fn create_shader_context(vertex_shader: &str, fragment_shader: &str) -> ShaderContext {
    let vertex_shaders = create_shader_programs(read_vertex_shaders_into_memory(), gl::VERTEX_SHADER);
    let fragment_shaders = create_shader_programs(read_fragment_shaders_into_memory(), gl::FRAGMENT_SHADER);

    let vertex_shader = vertex_shaders.get(vertex_shader)
        .expect(&format!("failed to load vertex shader: {}", vertex_shader));

    let fragment_shader = fragment_shaders.get(fragment_shader)
        .expect(&format!("failed to load fragment shader: {}", fragment_shader));

    let program = create_program(&vertex_shader, &fragment_shader);

    ShaderContext{
        fragment_shaders,
        vertex_shaders,
        programs: HashMap::from([(String::from("basic"), program)])
    }
}

fn asset_entities(shader_context: &ShaderContext, object_context: &ObjectContext) -> Vec<Entity3d> {
    let file_content = object_context.objects.get("casette.obj").expect("file not found");
    let points = wavefront_object_as_points(file_content);
    let cassette = Entity3d::with_default_transform(
        &shader_context,
        "basic",
        points,
    );
    vec![cassette]
}

fn game_render_event(game_context: &mut GameContext<GameState>) {
    //let mut rotation = &mut game_context.entity_context.entities[0].transform.rotation;
    //rotation.x = rotation.x + 0.01;
    //rotation.y = rotation.y + 0.01;
    //rotation.z = rotation.z + 0.01;
}

fn init_game() -> (GameContext<GameState>, GlfwReceiver<(f64, WindowEvent)>) {
    let keymap = create_keymap();

    let (window_context, events)
        = init_opengl_window_manager();

    //let mut entities: Vec<Entity3d> = entities(&shader_context);

    let keyboard_context = KeyboardContext{
        keymap
    };

    let game_state = GameState{
        mode: OBJECT
    };

    let game = Game{
        state: game_state
    };

    let object_context = ObjectContext {
        objects: read_object_files_into_memory()
    };

    let shader_context = create_shader_context("basic.vert", "basic.frag");

    //let mut entities: Vec<Entity3d> = entities(&shader_context);
    let mut entities: Vec<Entity3d> = asset_entities(&shader_context, &object_context);

    let entity_context = EntityContext{
        entities,
        cameras: vec![Camera{
            transform: Transform::new_zero_transform_with_position(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 400.0,
            }),
            target: Vector3d::zero_vector()
        }]
    };

    let mouse_context = MouseContext{
        sensitivity: 10.0
    };

    (GameContext {
        shader_context,
        window_context,
        object_context,
        entity_context,
        keyboard_context,
        mouse_context,
        game,
    }, events)
}

pub fn start() {
    let (game_context, events) = init_game();

    start_opengl_window_manager(
        game_context,
        events,
        game_render_event,
        glfw_press_handler,
        glfw_cursor_handler
    )
}