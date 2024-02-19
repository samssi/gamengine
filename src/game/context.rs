#[derive(PartialEq)]
pub enum Mode {
    CAMERA,
    OBJECT
}
pub struct GameState {
    pub mode: Mode
}