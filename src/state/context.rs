use glfw::PWindow;

pub struct WindowManagerContext<'context> {
    pub window: &'context mut PWindow,
}

impl <'context> WindowManagerContext<'context> {
    pub fn new(window: &'context mut PWindow) -> Self {
        Self {
            window
        }
    }

}