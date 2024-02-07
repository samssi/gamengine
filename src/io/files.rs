use std::fs;

pub fn read_file_to_string(filepath: &str) -> String {
    return fs::read_to_string(filepath).unwrap();
}