use std::collections::HashMap;
use glfw::{GlfwReceiver, WindowEvent};
use crate::game::context::GameState;
use crate::game::level::{Level, Scene};
use crate::game::context::Mode::{CAMERA, OBJECT};
use crate::game::keyboard_handler::glfw_press_handler;
use crate::game::window_handler::glfw_cursor_handler;
use crate::graphics::opengl::{create_program, create_shader_programs};
use crate::io::keyboard::{create_keymap};
use crate::io::loader::{read_fragment_shaders_into_memory, read_object_files_into_memory, read_texture_images_into_memory, read_vertex_shaders_into_memory};
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
        programs: HashMap::from([(String::from("textured"), program)])
    }
}

fn game_render_event(game_context: &mut GameContext<GameState>) {
    /*for mut entity in &mut game_context.entity_context.entities {
        let mut rotation = &mut entity.transform.rotation;
        rotation.x = rotation.x + 1.01;
        rotation.y = rotation.y + 1.01;
        rotation.z = rotation.z + 1.01;
    }*/
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

    let shader_context = create_shader_context("textured.vert", "textured.frag");
    //let mut entities: Vec<Entity3d> = generate_cube_space(&shader_context, &object_context);
    let level1 = Scene::load(&object_context, &shader_context);
    println!("Rendering {} entities.", level1.entities.len());

    let entity_context = EntityContext{
        entities: level1.entities,
        cameras: level1.cameras
    };

    let mouse_context = MouseContext{
        x_sensitivity: 1000.0,
        y_sensitivity: 5000.0
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