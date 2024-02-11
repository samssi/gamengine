struct WavefrontObject {
    o: String,
    v: Vec<Vec<f32>>,
    f: Vec<Vec<Vec<f32>>>
}

fn push_to_acc<'a>(pattern: &str, value: &'a str, acc: &mut Vec<&'a str>) {
    if value.starts_with(pattern) { acc.push(value.strip_prefix(pattern).expect(&format!("{} prefix not found", pattern))) };
}

fn parse_v(vs: Vec<&str>) -> Vec<Vec<f32>> {
    vs.into_iter()
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

pub fn parse_points_from_object_file(file_content: &String) /*-> WavefrontObject*/ {
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

    let parsed_v = parse_v(v);
    let parsed_f = parse_f(f);
    println!("{:?}", parsed_v);
    println!("{:?}", parsed_f);
}