use crate::{
    cpu::intersect::{intersect_aabb, intersect_triangle},
    types::{HitRecord, Mat4, Ray, RayType, Vec3, Vertex, AABB},
};
#[repr(C)]
pub struct Node {
    pub aabb: AABB,
    pub first_primitive: u32,
    pub primitive_count: u32,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            first_primitive: 0,
            primitive_count: 0,
            aabb: AABB {
                min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
                max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
            },
        }
    }
}
pub struct BottomLevelAccelerationStructure {
    vertices: Vec<Vertex>,
    triangles: Vec<u32>,
    indices: Vec<u32>,
    nodes: Vec<Node>,
}

impl BottomLevelAccelerationStructure {
    pub fn new(vertices: &[Vertex], indices: &[u32]) -> Self {
        // Initialize all nodes to default
        let mut centroids = Vec::new();
        let mut nodes = Vec::new();
        let mut triangle_indices = Vec::new();

        for i in (0..indices.len()).step_by(3) {
            triangle_indices.push(i as u32);
            nodes.extend([Node::default(), Node::default()].into_iter());
            let c = vertices[indices[i] as usize]
                + vertices[indices[i + 1] as usize]
                + vertices[indices[i + 2] as usize];
            centroids.push(Vec3::new(c.x, c.y, c.z) / 3.0)
        }

        // Root node contains all primitives
        nodes[0].primitive_count = triangle_indices.len() as u32;

        let mut used_nodes = 1;
        Self::update_bounds(vertices, &triangle_indices, indices, &mut nodes, 0);
        Self::subdivide(
            &mut nodes,
            0,
            vertices,
            &mut triangle_indices,
            indices,
            &centroids,
            &mut used_nodes,
        );
        Self {
            nodes,
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
            triangles: triangle_indices,
        }
    }

    pub fn aabb(&self) -> &AABB {
        &self.nodes[0].aabb
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn triangles(&self) -> &[u32] {
        &self.triangles
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    fn update_bounds(
        vertices: &[Vertex],
        triangle_indices: &[u32],
        vertex_indices: &[u32],
        nodes: &mut [Node],
        idx: usize,
    ) {
        let node = &mut nodes[idx];
        let first = node.first_primitive as usize;
        let last = first + node.primitive_count as usize;
        (first..last).for_each(|i| {
            let t = triangle_indices[i] as usize;
            let v0 = vertex_indices[t] as usize;
            let v1 = vertex_indices[t + 1] as usize;
            let v2 = vertex_indices[t + 2] as usize;
            node.aabb.min = crate::types::min(&node.aabb.min, &vertices[v0]);
            node.aabb.min = crate::types::min(&node.aabb.min, &vertices[v1]);
            node.aabb.min = crate::types::min(&node.aabb.min, &vertices[v2]);

            node.aabb.max = crate::types::max(&node.aabb.max, &vertices[v0]);
            node.aabb.max = crate::types::max(&node.aabb.max, &vertices[v1]);
            node.aabb.max = crate::types::max(&node.aabb.max, &vertices[v2]);
        });
    }

    fn find_split_axis(
        node: &Node,
        vertices: &[Vertex],
        triangle_indices: &[u32],
        vertex_indices: &[u32],
        centroids: &[Vec3],
    ) -> (usize, f32, f32) {
        let mut best_axis = 0;
        let mut best_position = 0.0;
        let mut best_cost = f32::MAX;

        let extent = node.aabb.extent();
        for axis in 0..3 {
            let min = node.aabb.min[axis];
            let max = node.aabb.max[axis];
            if min == max {
                continue;
            }
            let bins = 8;
            let scale = extent[axis] / bins as f32;
            for i in 0..bins {
                let candidate = min + i as f32 * scale;
                let cost = Self::evaluate_sah(
                    node,
                    vertices,
                    triangle_indices,
                    vertex_indices,
                    centroids,
                    axis,
                    candidate,
                );
                if cost < best_cost {
                    best_position = candidate;
                    best_axis = axis;
                    best_cost = cost;
                }
            }
        }

        (best_axis, best_position, best_cost)
    }

    fn subdivide(
        nodes: &mut [Node],
        idx: usize,
        vertices: &[Vertex],
        triangle_indices: &mut [u32],
        vertex_indices: &[u32],
        centroids: &[Vec3],
        used_nodes: &mut usize,
    ) {
        let node = &nodes[idx];
        let (axis, split, cost) =
            Self::find_split_axis(node, vertices, triangle_indices, vertex_indices, centroids);
        let parent_area = node.aabb.area();
        let parent_cost = node.primitive_count as f32 * parent_area;
        // Only split if costs are lower or equal to parent
        if cost >= parent_cost {
            return;
        }

        let mut i = node.first_primitive as i64;
        let mut j = i + node.primitive_count as i64 - 1;
        while i <= j {
            if centroids[triangle_indices[i as usize] as usize / 3][axis] < split {
                i += 1;
            } else {
                triangle_indices.swap(i as usize, j as usize);
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

        Self::update_bounds(
            vertices,
            triangle_indices,
            vertex_indices,
            nodes,
            left_child_index,
        );
        Self::update_bounds(
            vertices,
            triangle_indices,
            vertex_indices,
            nodes,
            right_child_index,
        );
        Self::subdivide(
            nodes,
            left_child_index,
            vertices,
            triangle_indices,
            vertex_indices,
            centroids,
            used_nodes,
        );
        Self::subdivide(
            nodes,
            right_child_index,
            vertices,
            triangle_indices,
            vertex_indices,
            centroids,
            used_nodes,
        );
    }

    fn evaluate_sah(
        node: &Node,
        vertices: &[Vertex],
        triangle_indices: &[u32],
        vertex_indices: &[u32],
        centroids: &[Vec3],
        axis: usize,
        position: f32,
    ) -> f32 {
        let mut left_box = AABB::default();
        let mut right_box = AABB::default();

        let mut left_count = 0;
        let mut right_count = 0;

        let first = node.first_primitive as usize;
        let count = node.primitive_count as usize;

        (first..first + count).for_each(|i| {
            let t = triangle_indices[i] as usize;
            let i0 = vertex_indices[t] as usize;
            let i1 = vertex_indices[t + 1] as usize;
            let i2 = vertex_indices[t + 2] as usize;
            let centroid = &centroids[t / 3];
            let v0 = vertices[i0];
            let v1 = vertices[i1];
            let v2 = vertices[i2];

            if centroid[axis] < position {
                left_count += 1;
                left_box.grow_with_position(&v0);
                left_box.grow_with_position(&v1);
                left_box.grow_with_position(&v2);
            } else {
                right_count += 1;
                right_box.grow_with_position(&v0);
                right_box.grow_with_position(&v1);
                right_box.grow_with_position(&v2);
            }
        });

        let cost = left_count as f32 * left_box.area() + right_count as f32 * right_box.area();
        if cost > 0.0 {
            cost
        } else {
            f32::MAX
        }
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<Node>() * self.nodes.len()
    }

    pub fn traverse(
        &self,
        ray: &Ray,
        ray_type: RayType,
        transform: &Mat4,
        hit_record: &mut HitRecord,
    ) {
        let mut node_idx = 0;
        let mut stack_ptr = 0;
        let mut stack = [0; 64];
        let inv_ray = ray.transformed(&transform.try_inverse().unwrap());
        loop {
            let node = &self.nodes[node_idx];
            if self.nodes[node_idx].primitive_count > 0 {
                let first = node.first_primitive as usize;
                let last = first + node.primitive_count as usize;
                for (_, p) in self.triangles[first..last].iter().enumerate() {
                    let mut t = 0.0;
                    let mut u = 0.0;
                    let mut v = 0.0;

                    let triangle = *p as usize;
                    let v0 = self.indices[triangle];
                    let v1 = self.indices[triangle + 1];
                    let v2 = self.indices[triangle + 2];
                    let hit = intersect_triangle(
                        &inv_ray,
                        &self.vertices[v0 as usize],
                        &self.vertices[v1 as usize],
                        &self.vertices[v2 as usize],
                        &mut t,
                        &mut u,
                        &mut v,
                    );
                    if hit && t < hit_record.t {
                        hit_record.t = t;
                        hit_record.u = u;
                        hit_record.v = v;
                        hit_record.primitive_id = triangle as _;
                        hit_record.ray = *ray;
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
            let mut left_distance = intersect_aabb(&left_child.aabb, &inv_ray, f32::MAX);
            let mut right_distance = intersect_aabb(&right_child.aabb, &inv_ray, f32::MAX);

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
    }
}
