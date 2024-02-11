use glfw::{GlfwReceiver, Key, WindowEvent};
use crate::entity::entity::{CUBE, Entity3d, Shading, TRIANGLE, Vector3d};
use crate::game::context::GameState;
use crate::game::keyboard_handler::glfw_press_handler;
use crate::io::keyboard::{create_keymap};
use crate::os::window_manager::{init_opengl_window_manager, start_opengl_window_manager};
use crate::state::context::{EntityContext, Game, GameContext, KeyboardContext, ShaderContext};

fn entities(shader_context: &ShaderContext) -> Vec<Entity3d> {
    let basic_shading = Shading{
        vertex_shader: String::from("basic.vert"),
        fragment_shader: String::from("basic.frag")};

    let triangle = Entity3d::with_default_transform(
        &shader_context,
        TRIANGLE.to_vec(),
        &basic_shading
    );

    let cube = Entity3d::with_default_transform(
        &shader_context,
        CUBE.to_vec(),
        &basic_shading
    );

    vec![triangle]
}
fn game_render_event(game_context: &mut GameContext<GameState>) {
    let mut rotation = &mut game_context.entity_context.entities[0].transform.rotation;
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

    (GameContext {
        shader_context,
        window_context,
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