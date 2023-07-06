use std::path::Path;

pub struct SceneImporter {}

impl SceneImporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn import_obj(&self, path: &Path) {
        let (models, materials) =
            tobj::load_obj(path, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");

        // Note: If you don't mind missing the materials, you can generate a default.
        let materials = materials.expect("Failed to load MTL file");

        println!("Number of models          = {}", models.len());
        println!("Number of materials       = {}", materials.len());

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            println!();
            println!("model[{}].name             = \'{}\'", i, m.name);
            println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

            println!(
                "model[{}].face_count       = {}",
                i,
                mesh.face_arities.len()
            );

            let mut next_face = 0;
            for face in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[face] as usize;

                let face_indices = &mesh.indices[next_face..end];
                println!(" face[{}].indices          = {:?}", face, face_indices);

                if !mesh.texcoord_indices.is_empty() {
                    let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
                    println!(
                        " face[{}].texcoord_indices = {:?}",
                        face, texcoord_face_indices
                    );
                }
                if !mesh.normal_indices.is_empty() {
                    let normal_face_indices = &mesh.normal_indices[next_face..end];
                    println!(
                        " face[{}].normal_indices   = {:?}",
                        face, normal_face_indices
                    );
                }

                next_face = end;
            }

            // Normals and texture coordinates are also loaded, but not printed in
            // this example.
            println!(
                "model[{}].positions        = {}",
                i,
                mesh.positions.len() / 3
            );
            assert!(mesh.positions.len() % 3 == 0);

            for vtx in 0..mesh.positions.len() / 3 {
                println!(
                    "              position[{}] = ({}, {}, {})",
                    vtx,
                    mesh.positions[3 * vtx],
                    mesh.positions[3 * vtx + 1],
                    mesh.positions[3 * vtx + 2]
                );
            }
        }

        for (i, m) in materials.iter().enumerate() {
            println!("material[{}].name = \'{}\'", i, m.name);
            if let Some(ambient) = m.ambient {
                println!(
                    "    material.Ka = ({}, {}, {})",
                    ambient[0], ambient[1], ambient[2]
                );
            }

            if let Some(diffuse) = m.diffuse {
                println!(
                    "    material.Kd = ({}, {}, {})",
                    diffuse[0], diffuse[1], diffuse[2]
                );
            }
            if let Some(specular) = m.specular {
                println!(
                    "    material.Kd = ({}, {}, {})",
                    specular[0], specular[1], specular[2]
                );
            }
            println!("    material.Ns = {}", m.shininess.unwrap_or(-1.0));
            println!("    material.d = {}", m.dissolve.unwrap_or(-1.0));
            println!(
                "    material.map_Ka = {}",
                m.ambient_texture.as_ref().unwrap_or(&"N.A".into())
            );
            println!(
                "    material.map_Kd = {}",
                m.diffuse_texture.as_ref().unwrap_or(&"N.A".into())
            );
            println!(
                "    material.map_Ks = {}",
                m.specular_texture.as_ref().unwrap_or(&"N.A".into())
            );
            println!(
                "    material.map_Ns = {}",
                m.shininess_texture.as_ref().unwrap_or(&"N.A".into())
            );
            println!(
                "    material.map_Bump = {}",
                m.normal_texture.as_ref().unwrap_or(&"N.A".into())
            );
            println!(
                "    material.map_d = {}",
                m.dissolve_texture.as_ref().unwrap_or(&"N.A".into())
            );

            for (k, v) in &m.unknown_param {
                println!("    material.{} = {}", k, v);
            }
        }
    }
}

impl Default for SceneImporter {
    fn default() -> Self {
        Self::new()
    }
}
