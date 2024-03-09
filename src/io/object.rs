pub struct WavefrontObject {
    o: String,
    v: Vec<Vec<f32>>,
    vt: Vec<Vec<f32>>,
    vn: Vec<Vec<f32>>,
    f: Vec<Vec<Vec<i32>>>
}

fn push_to_acc<'a>(pattern: &str, value: &'a str, acc: &mut Vec<&'a str>) {
    if value.starts_with(pattern) { acc.push(value.strip_prefix(pattern).expect(&format!("{} prefix not found", pattern))) };
}

fn parse_values(values: Vec<&str>) -> Vec<Vec<f32>> {
    values.into_iter()
       .map(|v| {
            v.split(" ")
             .map(|value| value.parse::<f32>().expect("failed to parse v value")).collect()
        }).collect()
}

fn parse_f(fs: Vec<&str>) -> Vec<Vec<Vec<i32>>> {
    let faces = fs.into_iter()
        .map(|v| {
            v.split(" ")
             .map(|value| {
                   value.split("/")
                        .map(|f| f.parse::<i32>().expect("failed to parse f value")).collect()
                }).collect()
        }).collect();
    faces
}

pub fn parse_wavefront_object_file(file_content: &String) -> WavefrontObject {
    let (o, v, vt, f, vn): (Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>, Vec<&str>) = file_content
        .lines()
        .fold( (vec![], vec![], vec![], vec![], vec![]), |mut acc, value| {
            push_to_acc("o ", value, &mut acc.0);
            push_to_acc("v ", value, &mut acc.1);
            push_to_acc("vt ", value, &mut acc.2);
            push_to_acc("f ", value, &mut acc.3);
            push_to_acc("vn ", value, &mut acc.4);
            acc
        });

    let parsed_v = parse_values(v);
    let parsed_f = parse_f(f);
    let parsed_vt = parse_values(vt);
    let parsed_vn = parse_values(vn);

    WavefrontObject{
        o: String::from(o[0]),
        v: parsed_v,
        vt: parsed_vt,
        vn: parsed_vn,
        f: parsed_f
    }
}

fn extract_data_from_wavefront_object(faces: Vec<Vec<Vec<i32>>>, coordinates: Vec<Vec<f32>>, index: usize) -> Vec<Vec<f32>> {
    let elements: Vec<Vec<i32>> = faces.to_vec().into_iter().flatten().collect();
    let positions: Vec<i32> = elements.into_iter().map(|element| element[index] - 1).collect();
    positions.into_iter().map(|position| {
        let triangles =  &coordinates[position as usize];
        triangles.to_vec()
    }).collect()
}

fn geometric_vertices_from_wavefront_object(wavefront_object: WavefrontObject) -> Vec<Vec<f32>> {
    extract_data_from_wavefront_object(wavefront_object.f, wavefront_object.v, 0)
}

fn texture_points_from_wavefront_object(wavefront_object: WavefrontObject) -> Vec<Vec<f32>> {
    extract_data_from_wavefront_object(wavefront_object.f, wavefront_object.vt, 1)
}

fn normals_from_wavefront_object(wavefront_object: WavefrontObject) -> Vec<Vec<f32>> {
    extract_data_from_wavefront_object(wavefront_object.f, wavefront_object.vn, 2)
}

pub fn get_geometric_vertices(wavefront_file_content: &String) -> Vec<f32> {
    let wavefront_object = parse_wavefront_object_file(wavefront_file_content);
    let geometric_vertices = geometric_vertices_from_wavefront_object(wavefront_object);
    geometric_vertices.into_iter().flatten().map(|item| item).collect()
}

pub fn get_texture_points(wavefront_file_content: &String) -> Vec<f32> {
    let wavefront_object = parse_wavefront_object_file(wavefront_file_content);
    let texture_points = texture_points_from_wavefront_object(wavefront_object);
    texture_points.into_iter().flatten().collect()
}

pub fn get_normals(wavefront_file_content: &String) -> Vec<f32> {
    let wavefront_object = parse_wavefront_object_file(wavefront_file_content);
    let normals = normals_from_wavefront_object(wavefront_object);
    normals.into_iter().flatten().collect()
}