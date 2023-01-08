use crate::{
    acceleration_structure::AccelerationStructure,
    bvh::BVHMidPointSplit,
    types::{Ray, Triangle, Vertex},
};

pub struct AccMidPointSplit {
    bvh: Option<BVHMidPointSplit>,
}

// Slits aabb into middle along the largest axis
impl AccMidPointSplit {
    pub fn new() -> Self {
        Self { bvh: None }
    }
}

impl AccelerationStructure for AccMidPointSplit {
    fn build(&mut self, vertices: &[Vertex], triangles: &[Triangle]) {
        self.bvh = Some(BVHMidPointSplit::new(vertices, triangles))
    }

    fn trace(&self, ray: &Ray) -> (i32, f32) {
        todo!()
    }
}
