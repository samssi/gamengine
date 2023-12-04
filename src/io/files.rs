pub fn read_file_to_string(shader_path: &str) -> String {
    return fs::read_to_string(shader_path)
        .expect(format!("File: {} not found!", shader_path).as_str());
}