pub mod cpu;
pub mod cube;
pub mod gpu;
pub mod ray_tracer;
pub mod types;

use std::io::BufRead;

use types::Vertex;

pub fn read_triangle_file(name: &str) -> (Vec<Vertex>, Vec<u32>) {
    let path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "/assets/"
        + name;
    let file = std::fs::File::open(path).expect("Couldn't open file");
    let reader = std::io::BufReader::new(file);
    let positions = reader
        .lines()
        .flat_map(|line| match line {
            Ok(line) => {
                let floats = line
                    .split(' ')
                    .map(|token| {
                        let v: f32 = token.parse().expect("float parse failed");
                        v
                    })
                    .collect::<Vec<f32>>();

                floats.into_iter()
            }

            Err(_) => todo!(),
        })
        .collect::<Vec<f32>>();

    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    for i in (0..positions.len()).step_by(9) {
        vertices.push(Vertex::new(
            positions[i],
            positions[i + 1],
            positions[i + 2],
        ));
        vertices.push(Vertex::new(
            positions[i + 3],
            positions[i + 4],
            positions[i + 5],
        ));
        vertices.push(Vertex::new(
            positions[i + 6],
            positions[i + 7],
            positions[i + 8],
        ));
    }

    for i in (0..vertices.len()).step_by(3) {
        triangles.push(i as u32);
        triangles.push(i as u32 + 1);
        triangles.push(i as u32 + 2);
    }

    (vertices, triangles)
}
