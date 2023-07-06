use std::rc::Rc;

use libraytracer::{
    cpu::{
        bvh::BottomLevelAccelerationStructure,
        shape::{Shape, SurfaceAttributes},
    },
    types::{HitRecord, Mat4, Ray, RayType, Vec2, Vertex, AABB},
};

pub struct TriangleMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub blas: Rc<BottomLevelAccelerationStructure>,
}

impl Shape for TriangleMesh {
    fn intersect(
        &self,
        ray: &Ray,
        ray_type: RayType,
        transform: &Mat4,
        t_max: f32,
    ) -> Option<HitRecord> {
        self.blas.traverse(ray, ray_type, transform, t_max)
    }

    fn aabb(&self) -> AABB {
        *self.blas.aabb()
    }

    fn surface_attributes(&self, hit_record: &HitRecord) -> SurfaceAttributes {
        let i = self.indices[hit_record.primitive_id as usize] as usize;
        let i0 = i;
        let i1 = i + 1;
        let i2 = i + 2;
        let v0 = self.vertices[i0];
        let v1 = self.vertices[i1];
        let v2 = self.vertices[i2];
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(&e2).normalize();

        SurfaceAttributes {
            position: hit_record.ray.origin + hit_record.ray.direction * hit_record.t,
            normal,
            uv: Vec2::new(0.0, 0.0),
        }
    }
}

impl TriangleMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let blas = Rc::new(BottomLevelAccelerationStructure::new(&vertices, &indices));
        Self {
            vertices,
            indices,
            blas,
        }
    }

    pub fn center(&mut self) {
        let mut bb = AABB::default();
        for vertex in &self.vertices {
            bb.grow_with_position(vertex)
        }

        let center = bb.centroid();
        self.vertices = self
            .vertices
            .iter()
            .map(|position| center - position)
            .collect();
    }
}
