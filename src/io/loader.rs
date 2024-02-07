use std::collections::HashMap;
use crate::io::files::read_file_to_string;

pub fn read_shaders_into_memory() -> HashMap<&'static str, String> {
    // TODO: read all shader dir contents into HashMap and return them
    let shaders = HashMap::from([
        ("basic.frag", read_file_to_string("assets/shaders/fragment/basic.frag")),
        ("basic.vert", read_file_to_string("assets/shaders/vertex/basic.vert"))
    ]);

    return shaders;
}