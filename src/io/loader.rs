use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use image::DynamicImage;


fn read_file_to_string(filepath: &str) -> String {
    fs::read_to_string(filepath).unwrap()
}
fn list_directory_files(dirname: &str) -> Vec<DirEntry> {
    let files = fs::read_dir(dirname).unwrap();

    files.filter_map(|file| {
        let dir_entry = file.unwrap();
        let file_type = dir_entry.file_type().ok()?;
        if file_type.is_file() { Some(dir_entry) } else { None }
    }).collect()
}

fn into_map(dir_entry: Vec<DirEntry>) -> HashMap<String, String> {
    let mut shaders: HashMap<String, String> =
        dir_entry
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<String, String>, entry| {
                let file_name = entry.file_name();
                let shader_path = entry.path();
                let shader_source = read_file_to_string(shader_path
                    .to_str()
                    .expect("file not found"));

                acc.entry(file_name.into_string().expect("filename not found"))
                    .or_insert(shader_source.to_owned());

                acc
            });

    return shaders;
}

fn read_file_as_image(filepath: &str) -> DynamicImage {
    image::open(filepath).unwrap()
}

fn into_image_map(dir_entry: Vec<DirEntry>) -> HashMap<String, DynamicImage> {
    let mut images: HashMap<String, DynamicImage> =
        dir_entry
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<String, DynamicImage>, entry| {
                let file_name = entry.file_name();
                let shader_path = entry.path();
                let image = read_file_as_image(shader_path
                    .to_str()
                    .expect("file not found"));

                acc.entry(file_name.into_string().expect("filename not found"))
                    .or_insert(image.to_owned());

                acc
            });

    return images;
}

pub fn read_object_files_into_memory() -> HashMap<String, String> {
    let object_files = list_directory_files("assets/objects");
    into_map(object_files)
}

pub fn read_fragment_shaders_into_memory() -> HashMap<String, String> {
    let fragment_shader_files = list_directory_files("assets/shaders/fragment");
    into_map(fragment_shader_files)
}

pub fn read_vertex_shaders_into_memory() -> HashMap<String, String> {
    let vertex_shader_files = list_directory_files("assets/shaders/vertex");
    into_map(vertex_shader_files)
}

pub fn read_texture_images_into_memory() -> HashMap<String, DynamicImage> {
    let texture_files = list_directory_files("assets/textures");
    into_image_map(texture_files)
}