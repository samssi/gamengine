mod state;
mod graphics;
mod os;
mod entity;
mod io;
mod game;
mod audio;

use std::thread;
use crate::game::game::start;

fn main() {
    let main_thread = thread::spawn(|| { start() });
    main_thread.join().unwrap();
}