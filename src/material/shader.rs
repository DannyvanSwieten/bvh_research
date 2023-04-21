pub trait Shader {
    fn uid(&self) -> u32;
    fn name(&self) -> &str;
    fn source(&self) -> &str;
}

pub struct DiffuseShader {}

impl DiffuseShader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Shader for DiffuseShader {
    fn uid(&self) -> u32 {
        0
    }

    fn name(&self) -> &str {
        "diffuse"
    }

    fn source(&self) -> &str {
        r#"
        Ray diffuse(int parameter_offset, instance_id, primitive_id, ray_in) -> vec4 {
            vec3 N = get_normal(instance_id, primitive_id);
            Ray ray_out;
            ray_out.origin = ray_in.origin + ray_in.direction * ray_in.t;
            ray_out.direction = random_in_unit_sphere();
            ray_out.color = ray_in.color * get_vec4(parameter_offset) * max(0.0, dot(ray.direction, N));

            return ray;
        }
        "#
    }
}

pub struct MirrorShader {}

impl MirrorShader {
    pub fn new() -> Self {
        Self {}
    }
}

impl Shader for MirrorShader {
    fn uid(&self) -> u32 {
        1
    }

    fn name(&self) -> &str {
        "mirror"
    }

    fn source(&self) -> &str {
        r#"
        Ray mirror(int parameter_offset, instance_id, primitive_id, wi, wo) -> vec4 {
            vec3 N = get_normal(instance_id, primitive_id);
            Ray ray_out;
            ray_out.origin = ray_in.origin + ray_in.direction * ray_in.t;
            ray_out.direction = reflect(-ray_in.direction, N);
            ray_out.color = ray_in.color * get_vec4(parameter_offset);

            return ray;
        }
        "#
    }
}
