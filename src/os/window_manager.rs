use glfw::Action;
use glfw::{Context, Key};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn report_run_count(count: i32) {
    println!("Running... Count {}", count)
}

pub fn start_window_manager() {
    println!("Starting window manager!");

    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) =
        glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, "GamEngine", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // let mut count = 0;

        while !window.should_close() {
            // count += 1;
            // report_run_count(count);
            window.swap_buffers();
            glfw.poll_events();

            for (_, event) in glfw::flush_messages(&events) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    },
                _ => {},
            }
        }
    }
}