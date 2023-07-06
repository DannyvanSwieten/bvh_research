use std::rc::Rc;

use crate::{
    cpu::{cpu_shader_binding_table::ShaderBindingTable, intersect::intersect_aabb},
    types::{HitRecord, Mat4, Ray, RayType, AABB},
};

use super::shape::Shape;

pub struct TlasNode {
    aabb: AABB,
    first_primitive: u32,
    primitive_count: u32,
}

impl TlasNode {
    pub fn new() -> Self {
        Self {
            aabb: AABB::default(),
            first_primitive: 0,
            primitive_count: 0,
        }
    }
}

impl Default for TlasNode {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Instance {
    pub blas: Rc<dyn Shape>,
    id: u32,
    transform: Mat4,
}

impl Instance {
    pub fn new(blas: Rc<dyn Shape>, id: u32, transform: Mat4) -> Self {
        Self {
            blas,
            id,
            transform,
        }
    }
}

pub struct TopLevelAccelerationStructure {
    nodes: Vec<TlasNode>,
    instances: Vec<Instance>,
    used_nodes: usize,
}

impl TopLevelAccelerationStructure {
    fn update_bounds(&mut self, idx: usize, boxes: &mut [AABB]) {
        let node = &self.nodes[idx];
        let first = node.first_primitive as usize;
        let last = first + node.primitive_count as usize;

        let mut aabb = AABB::default();
        (first..last).for_each(|i| aabb.grow(&boxes[i]));

        self.nodes[idx].aabb = aabb;
    }

    fn subdivide(&mut self, idx: usize, boxes: &mut [AABB]) {
        let node = &self.nodes[idx];

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
                self.instances.swap(i as usize, j as usize);
                j -= 1;
            }
        }

        let left_count = i as usize - node.first_primitive as usize;
        if left_count == 0 || left_count == node.primitive_count as usize {
            return;
        }

        let left_child_index = self.used_nodes;
        self.used_nodes += 1;
        let right_child_index = self.used_nodes;
        self.used_nodes += 1;
        self.nodes[left_child_index].first_primitive = self.nodes[idx].first_primitive;
        self.nodes[left_child_index].primitive_count = left_count as u32;
        self.nodes[right_child_index].first_primitive = i as u32;
        self.nodes[right_child_index].primitive_count =
            self.nodes[idx].primitive_count - left_count as u32;
        self.nodes[idx].first_primitive = left_child_index as u32;
        self.nodes[idx].primitive_count = 0;

        self.update_bounds(left_child_index, boxes);
        self.update_bounds(right_child_index, boxes);
        self.subdivide(left_child_index, boxes);
        self.subdivide(right_child_index, boxes);
    }

    pub fn new(instances: &[Instance]) -> Self {
        let mut nodes = Vec::new();
        let mut boxes = Vec::new();
        for instance in instances {
            nodes.push(TlasNode::new());
            boxes.push(instance.blas.aabb().transformed(&instance.transform))
        }

        nodes[0].primitive_count = instances.len() as u32;
        let mut this = Self {
            used_nodes: 1,
            nodes,
            instances: instances.to_vec(),
        };
        this.update_bounds(0, &mut boxes);
        if instances.len() == 2 {
            this.nodes[0].first_primitive = 0;
            this.nodes[0].primitive_count = 2;
            this
        } else {
            this.subdivide(0, &mut boxes);
            this
        }
    }

    pub fn size(&self) -> u64 {
        std::mem::size_of::<TlasNode>() as u64 * self.nodes.len() as u64
    }

    pub fn nodes(&self) -> &[TlasNode] {
        &self.nodes
    }

    pub fn trace<Context, Payload>(
        &self,
        ctx: &Context,
        sbt: &ShaderBindingTable<Context, Payload>,
        ray: &Ray,
        ray_type: RayType,
        payload: &mut Payload,
        miss_shader_index: usize,
        ray_depth: u32,
    ) -> Option<HitRecord> {
        self.traverse_stack(
            ctx,
            sbt,
            ray,
            ray_type,
            payload,
            miss_shader_index,
            ray_depth,
        )
    }

    fn traverse_stack<Context, Payload>(
        &self,
        ctx: &Context,
        sbt: &ShaderBindingTable<Context, Payload>,
        ray: &Ray,
        ray_type: RayType,
        payload: &mut Payload,
        miss_shader_index: usize,
        ray_depth: u32,
    ) -> Option<HitRecord> {
        let mut node_idx = 0;
        let mut stack_ptr = 0;
        let mut stack = [0; 64];
        let mut d = f32::MAX;
        let mut hits = Vec::new();
        loop {
            let node = &self.nodes[node_idx];
            if self.nodes[node_idx].primitive_count > 0 {
                let first = node.first_primitive as usize;
                let last = first + node.primitive_count as usize;

                for i in first..last {
                    let instance = &self.instances[i];
                    let transform = &instance.transform;
                    if let Some(mut hit_record) =
                        instance.blas.intersect(ray, ray_type, transform, f32::MAX)
                    {
                        hit_record.ray_depth = ray_depth;
                        hit_record.object_id = instance.id;
                        d = hit_record.t;
                        hit_record.obj_to_world = *transform;
                        hits.push(hit_record);
                        if let RayType::Shadow = ray_type {
                            break;
                        }
                    }
                }

                if stack_ptr == 0 {
                    break;
                } else {
                    stack_ptr -= 1;
                    node_idx = stack[stack_ptr];
                    continue;
                }
            }

            let mut left_child_idx = node.first_primitive as usize;
            let mut right_child_idx = left_child_idx + 1;
            let left_child = &self.nodes[left_child_idx];
            let right_child = &self.nodes[right_child_idx];
            let mut left_distance = intersect_aabb(&left_child.aabb, ray, f32::MAX);
            let mut right_distance = intersect_aabb(&right_child.aabb, ray, f32::MAX);
            if left_distance > d || right_distance > d {
                if stack_ptr == 0 {
                    break;
                } else {
                    stack_ptr -= 1;
                    node_idx = stack[stack_ptr];
                    continue;
                }
            }
            if left_distance > right_distance {
                std::mem::swap(&mut left_child_idx, &mut right_child_idx);
                std::mem::swap(&mut left_distance, &mut right_distance);
            }
            if left_distance == f32::MAX {
                if stack_ptr == 0 {
                    break;
                } else {
                    stack_ptr -= 1;
                    node_idx = stack[stack_ptr];
                }
            } else {
                node_idx = left_child_idx;
                if right_distance != f32::MAX {
                    stack[stack_ptr] = right_child_idx;
                    stack_ptr += 1;
                }
            }
        }

        let record = hits
            .into_iter()
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        if let Some(record) = record {
            match ray_type {
                RayType::Primary => {
                    sbt.closest_hit_shader(record.closest_hit_shader as usize)
                        .execute(ctx, payload, &record);
                }

                RayType::Shadow => {
                    sbt.any_hit_shader().execute(ctx, payload, &record);
                }
                RayType::Reflection => todo!(),
                RayType::Refraction => todo!(),
            }
        } else {
            sbt.miss_shader(miss_shader_index)
                .execute(ctx, payload, ray);
        }

        record
    }

    pub fn instances(&self) -> &[Instance] {
        &self.instances
    }
}

unsafe impl Sync for TopLevelAccelerationStructure {}
