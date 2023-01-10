use crate::{
    acceleration_structure::AccelerationStructure,
    bvh::BVHMidPointSplit,
    types::{Ray, Triangle, Vertex},
};

pub struct AccMidPointSplit {
    bvh: Option<BVHMidPointSplit>,
    use_sah: bool,
}

// Slits aabb into middle along the largest axis
impl AccMidPointSplit {
    pub fn new(use_sah: bool) -> Self {
        Self { bvh: None, use_sah }
    }
}

impl Default for AccMidPointSplit {
    fn default() -> Self {
        Self::new(false)
    }
}

impl AccelerationStructure for AccMidPointSplit {
    fn build(&mut self, vertices: &[Vertex], triangles: &[Triangle]) {
        self.bvh = Some(BVHMidPointSplit::new(vertices, triangles, self.use_sah))
    }

    fn trace(&self, ray: &Ray) -> (i32, f32) {
        let bvh = self.bvh.as_ref().unwrap();
        (0, bvh.traverse_stack(ray))
    }
}
