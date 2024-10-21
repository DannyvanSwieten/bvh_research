use std::{mem::size_of, rc::Rc};

use vk_utils::{
    buffer_resource::BufferResource, device_context::DeviceContext, BufferUsageFlags,
    MemoryPropertyFlags,
};

use crate::{
    bvh::Node,
    types::{Mat4, AABB},
};

use super::{blas::Geometry, instance::Instance};

pub struct GpuTlas {
    pub tlas_buffer: BufferResource,
    pub instance_buffer: BufferResource,
}

#[repr(C)]
struct GpuInstance {
    blas: u64,
    instance_id: u32,
    flags: u32,
    transform: Mat4,
}

impl GpuTlas {
    pub fn new(device: Rc<DeviceContext>, proxies: &[Instance]) -> Self {
        let mut nodes = Vec::new();
        let mut boxes = Vec::new();
        for instance in proxies {
            match instance.blas() {
                Geometry::Triangle(_) => {
                    nodes.push(Node::default());
                }
                Geometry::Procedural(procedural) => {
                    nodes.push(Node::default().with_intersection_function_offset(
                        procedural.intersection_function_offset(),
                    ));
                }
            }
            boxes.push(instance.aabb().transformed(instance.transform()))
        }

        let mut instances: Vec<GpuInstance> = proxies
            .iter()
            .map(|proxy| GpuInstance {
                blas: proxy.address(),
                instance_id: proxy.id(),
                flags: if let Geometry::Procedural(p) = proxy.blas() {
                    p.intersection_function_offset()
                } else {
                    0
                },
                transform: *proxy.transform(),
            })
            .collect();

        nodes[0].primitive_count = proxies.len() as u32;
        let mut used_nodes = 1;
        Self::update_bounds(0, &mut nodes, &mut boxes);
        if nodes[0].primitive_count > 2 {
            Self::subdivide(0, &mut used_nodes, &mut nodes, &mut instances, &mut boxes);
        }

        let mut instance_buffer = BufferResource::new(
            device.clone(),
            std::mem::size_of_val(&instances),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );
        instance_buffer.upload(&instances);

        let mut tlas_buffer = BufferResource::new(
            device,
            std::mem::size_of_val(&nodes),
            MemoryPropertyFlags::HOST_VISIBLE,
            BufferUsageFlags::STORAGE_BUFFER,
        );

        tlas_buffer.upload(&nodes);

        Self {
            tlas_buffer,
            instance_buffer,
        }
    }

    fn update_bounds(idx: usize, nodes: &mut [Node], boxes: &mut [AABB]) {
        let node = &nodes[idx];
        let first = node.first_primitive as usize;
        let last = first + node.primitive_count as usize;

        let mut aabb = AABB::default();
        (first..last).for_each(|i| aabb.grow(&boxes[i]));

        nodes[idx].aabb = aabb;
    }

    fn subdivide(
        idx: usize,
        used_nodes: &mut usize,
        nodes: &mut [Node],
        instances: &mut [GpuInstance],
        boxes: &mut [AABB],
    ) {
        let node = &nodes[idx];

        let extent = node.aabb.extent();
        let axis = node.aabb.dominant_axis();
        let split = node.aabb.min[axis] + extent[axis] * 0.5;

        let mut i = node.first_primitive as i64;
        let mut j = i + node.primitive_count as i64 - 1;
        while i <= j {
            if boxes[i as usize].centroid()[axis] < split {
                i += 1;
            } else {
                boxes.swap(i as usize, j as usize);
                instances.swap(i as usize, j as usize);
                j -= 1;
            }
        }

        let left_count = i as usize - node.first_primitive as usize;
        if left_count == 0 || left_count == node.primitive_count as usize {
            return;
        }

        let left_child_index = *used_nodes;
        *used_nodes += 1;
        let right_child_index = *used_nodes;
        *used_nodes += 1;
        nodes[left_child_index].first_primitive = nodes[idx].first_primitive;
        nodes[left_child_index].primitive_count = left_count as u32;
        nodes[right_child_index].first_primitive = i as u32;
        nodes[right_child_index].primitive_count = nodes[idx].primitive_count - left_count as u32;
        nodes[idx].first_primitive = left_child_index as u32;
        nodes[idx].primitive_count = 0;

        Self::update_bounds(left_child_index, nodes, boxes);
        Self::update_bounds(right_child_index, nodes, boxes);
        Self::subdivide(left_child_index, used_nodes, nodes, instances, boxes);
        Self::subdivide(right_child_index, used_nodes, nodes, instances, boxes);
    }
}
