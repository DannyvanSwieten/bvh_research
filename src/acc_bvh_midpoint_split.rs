use core::panic;

use crate::{
    bottom_level_acceleration_structure::AccelerationStructure,
    bvh::{BVHMidPointSplit, Node},
    types::{Mat4, Ray, Triangle, Vertex},
};

pub struct AccMidPointSplit {
    bvh: Option<BVHMidPointSplit>,
}

// Slits aabb into middle along the largest axis
impl AccMidPointSplit {
    pub fn new(vertices: &[Vertex], triangles: &[Triangle], use_sah: bool) -> Self {
        Self {
            bvh: Some(BVHMidPointSplit::new(vertices, triangles, use_sah)),
        }
    }

    pub fn nodes(&self) -> &[Node] {
        if let Some(bvh) = &self.bvh {
            bvh.nodes()
        } else {
            panic!()
        }
    }

    pub fn triangles(&self) -> &[Triangle] {
        if let Some(bvh) = &self.bvh {
            bvh.triangles()
        } else {
            panic!()
        }
    }

    pub fn byte_size(&self) -> u64 {
        std::mem::size_of::<Node>() as u64 * self.nodes().len() as u64
    }
}

impl AccelerationStructure for AccMidPointSplit {
    fn trace(&self, ray: &Ray, transform: &Mat4) -> (i32, f32) {
        let bvh = self.bvh.as_ref().unwrap();
        (0, bvh.traverse_stack(ray, transform))
    }

    fn aabb(&self) -> &crate::types::AABB {
        &self.nodes()[0].aabb
    }
}
