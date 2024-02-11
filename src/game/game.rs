use glfw::{GlfwReceiver, Key, WindowEvent};
use crate::entity::entity::{Entity3d, f_letter_entity, Shading, TRIANGLE, Vector3d};
use crate::game::context::GameState;
use crate::game::keyboard_handler::glfw_press_handler;
use crate::io::keyboard::{create_keymap};
use crate::io::loader::read_object_files_into_memory;
use crate::os::window_manager::{init_opengl_window_manager, start_opengl_window_manager};
use crate::state::context::{EntityContext, Game, GameContext, KeyboardContext, ObjectContext, ShaderContext};

fn entities(shader_context: &ShaderContext) -> Vec<Entity3d> {
    let basic_shading = Shading{
        vertex_shader: String::from("basic.vert"),
        fragment_shader: String::from("basic.frag")};

    let triangle = Entity3d::with_position(
        &shader_context,
        TRIANGLE.to_vec(),
        &basic_shading,
        Vector3d{
            x: -200.0,
            y: -200.0,
            z: 0.0
        }
    );

    let f_letter = Entity3d::with_default_transform(
        &shader_context,
        f_letter_entity(),
        &basic_shading
    );

    vec![f_letter, triangle]
}
fn game_render_event(game_context: &mut GameContext<GameState>) {
    let mut rotation = &mut game_context.entity_context.entities[0].transform.rotation;
    rotation.x = rotation.x + 0.01;
    rotation.y = rotation.y + 0.01;
    rotation.z = rotation.z + 0.01;
}

fn init_game() -> (GameContext<GameState>, GlfwReceiver<(f64, WindowEvent)>) {
    let keymap = create_keymap();

    let (window_context, events, shader_context)
        = init_opengl_window_manager();

    let mut entities: Vec<Entity3d> = entities(&shader_context);

    let entity_context = EntityContext{
        entities
    };

    let keyboard_context = KeyboardContext{
        keymap
    };

    let game_state = GameState{
    };

    let game = Game{
        state: game_state
    };

    let object_context = ObjectContext{
        objects: read_object_files_into_memory()
    };

    (GameContext {
        shader_context,
        window_context,
        object_context,
        entity_context,
        keyboard_context,
        game,
    }, events)
}

pub fn start() {
    let (game_context, events) = init_game();

    start_opengl_window_manager(
        game_context,
        events,
        game_render_event,
        glfw_press_handler
    )
}