use crate::types::AABB;

pub struct ProceduralGeometry {
    aabb: AABB,
    intersection_function_offset: u32,
}

impl ProceduralGeometry {
    pub fn new(aabb: AABB, intersection_function_offset: u32) -> Self {
        Self {
            aabb,
            intersection_function_offset,
        }
    }

    pub fn aabb(&self) -> &AABB {
        &self.aabb
    }

    pub fn intersection_function_offset(&self) -> u32 {
        self.intersection_function_offset
    }
}
