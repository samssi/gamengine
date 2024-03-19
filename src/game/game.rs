use std::collections::HashMap;
use glfw::{GlfwReceiver, WindowEvent};
use rand::prelude::*;
use crate::game::context::GameState;
use crate::game::level::{leppis_default_transform};
use crate::game::context::Mode::{CAMERA, OBJECT};
use crate::game::keyboard_handler::glfw_press_handler;
use crate::game::scene::Scene;
use crate::game::window_handler::glfw_cursor_handler;
use crate::graphics::opengl::{create_program, create_shader_programs};
use crate::io::keyboard::{create_keymap};
use crate::io::loader::{read_fragment_shaders_into_memory, read_object_files_into_memory, read_texture_images_into_memory, read_vertex_shaders_into_memory};
use crate::os::window_manager::{init_opengl_window_manager, start_opengl_window_manager};
use crate::state::context::{EntityContext, Game, GameContext, KeyboardContext, MouseContext, ObjectContext, ShaderContext};

fn create_shader_context() -> ShaderContext {
    let vertex_shaders = create_shader_programs(read_vertex_shaders_into_memory(), gl::VERTEX_SHADER);
    let fragment_shaders = create_shader_programs(read_fragment_shaders_into_memory(), gl::FRAGMENT_SHADER);

    let vertex_shader = vertex_shaders.get("textured.vert")
        .expect(&format!("failed to load vertex shader: {}", "textured.vert"));

    let fragment_shader = fragment_shaders.get("textured.frag")
        .expect(&format!("failed to load fragment shader: {}", "textured.frag"));

    let program = create_program(&vertex_shader, &fragment_shader);

    ShaderContext{
        fragment_shaders,
        vertex_shaders,
        programs: HashMap::from([
            (String::from("textured"), program)])
    }
}

enum Axis {
    Y,
    Z
}

fn random_shake_value() -> f32 {
    let mut random = thread_rng();
    let random_value = if random.gen_bool(0.5) {-0.008} else {0.008};
    random_value
}

const SHAKE_FACTOR: f32 = 0.005;
const MOVE_LIMIT: f32 = 0.008;

fn shake_leppis(current_pos: f32, axis: Axis) -> f32 {
    let defaults = leppis_default_transform();

    match axis {
        Axis::Y => {
            if current_pos > (defaults.position.y + MOVE_LIMIT) {
                -SHAKE_FACTOR
            }
            else if current_pos < (defaults.position.y - MOVE_LIMIT) {
                SHAKE_FACTOR
            }
            else {
                random_shake_value()
            }
        }
        Axis::Z => {
            if current_pos > (defaults.position.z + MOVE_LIMIT) {
                -SHAKE_FACTOR
            }
            else if current_pos < (defaults.position.z - MOVE_LIMIT) {
                SHAKE_FACTOR
            }
            else {
                random_shake_value()
            }
        }
    }

}

fn game_render_event(game_context: &mut GameContext<GameState>) {
    /*let leppis_pos = &mut game_context.entity_context.entities[1].transform.position;
    leppis_pos.y = leppis_pos.y + shake_leppis(leppis_pos.y, Axis::Y);
    leppis_pos.z = leppis_pos.z + shake_leppis(leppis_pos.z, Axis::Z);*/
}

fn init_game() -> (GameContext<GameState>, GlfwReceiver<(f64, WindowEvent)>) {
    let keymap = create_keymap();

    let (window_context, events)
        = init_opengl_window_manager();

    let keyboard_context = KeyboardContext{
        keymap
    };

    let game_state = GameState{
        mode: OBJECT,
        lock_to_window: true
    };

    let game = Game{
        state: game_state
    };

    let object_context = ObjectContext {
        objects: read_object_files_into_memory(),
        textures: read_texture_images_into_memory()
    };

    let shader_context = create_shader_context();
    //let mut entities: Vec<Entity3d> = generate_cube_space(&shader_context, &object_context);
    //let level1 = Scene::load(&object_context, &shader_context);
    let scene = Scene::load();
    println!("Rendering {} entities.", scene.entities.len());

    let entity_context = EntityContext{
        entities: scene.entities,
        cameras: scene.cameras
    };

    let mouse_context = MouseContext{
        x_sensitivity: 1000.0,
        y_sensitivity: 1000.0
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
    let (mut game_context, events) = init_game();
    // let audio_thread1 = thread::spawn(|| { play_audio() });


    start_opengl_window_manager(
        &mut game_context,
        events,
        game_render_event,
        glfw_press_handler,
        glfw_cursor_handler
    );

    // audio_thread1.join().unwrap();
}