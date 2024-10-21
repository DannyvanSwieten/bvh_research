use std::rc::Rc;

use vk_utils::{buffer_resource::BufferResource, device_context::DeviceContext};

use crate::types::AABB;

use super::{procedural_blas::ProceduralGeometry, triangle_blas::TriangleGeometry};

pub enum Geometry {
    Triangle(TriangleGeometry),
    Procedural(ProceduralGeometry),
}

impl Geometry {
    pub fn aabb(&self) -> &AABB {
        match self {
            Geometry::Triangle(triangle) => triangle.aabb(),
            Geometry::Procedural(procedural) => procedural.aabb(),
        }
    }

    pub fn address(&self) -> u64 {
        match self {
            Geometry::Triangle(triangle) => triangle.address(),
            Geometry::Procedural(_) => 0,
        }
    }

    pub fn new_triangles(
        device: Rc<DeviceContext>,
        vertex_buffer: &BufferResource,
        index_buffer: &BufferResource,
    ) -> Self {
        Geometry::Triangle(TriangleGeometry::new(device, vertex_buffer, index_buffer))
    }

    pub fn new_procedural(aabb: AABB, intersection_function_offset: u32) -> Self {
        Geometry::Procedural(ProceduralGeometry::new(aabb, intersection_function_offset))
    }
}
