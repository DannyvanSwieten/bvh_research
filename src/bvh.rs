use crate::{
    intersect::{intersect_aabb, intersect_triangle},
    types::{vec4_to_3, Ray, Triangle, Vec3, Vertex, AABB},
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
pub struct BVHMidPointSplit {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    nodes: Vec<Node>,
    used_nodes: usize,
    use_sah: bool,
}

impl BVHMidPointSplit {
    pub fn new(vertices: &[Vertex], triangles: &[Triangle], use_sah: bool) -> Self {
        // Initialize all nodes to default
        let mut nodes = Vec::new();
        for _ in 0..triangles.len() * 2 {
            nodes.push(Node::default())
        }

        // Root node contains all primitives
        nodes[0].primitive_count = triangles.len() as u32;

        let mut centroids: Vec<Vec3> = triangles
            .iter()
            .map(|triangle| {
                let c = (vertices[triangle.v0 as usize]
                    + vertices[triangle.v1 as usize]
                    + vertices[triangle.v2 as usize])
                    / 3.0;
                Vec3::new(c.x, c.y, c.z)
            })
            .collect();

        let mut this = Self {
            nodes,
            vertices: vertices.to_vec(),
            triangles: triangles.to_vec(),
            used_nodes: 1,
            use_sah,
        };
        this.update_bounds(0);
        this.subdivide(0, &mut centroids);
        this
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn triangles(&self) -> &[Triangle] {
        &self.triangles
    }

    fn update_bounds(&mut self, idx: usize) {
        let mut node = &mut self.nodes[idx];
        let first = node.first_primitive as usize;
        let last = first + node.primitive_count as usize;
        for i in first..last as usize {
            node.aabb.min = crate::types::min(
                &node.aabb.min,
                &vec4_to_3(self.vertices[self.triangles[i].v0 as usize]),
            );
            node.aabb.min = crate::types::min(
                &node.aabb.min,
                &vec4_to_3(self.vertices[self.triangles[i].v1 as usize]),
            );
            node.aabb.min = crate::types::min(
                &node.aabb.min,
                &vec4_to_3(self.vertices[self.triangles[i].v2 as usize]),
            );

            node.aabb.max = crate::types::max(
                &node.aabb.max,
                &vec4_to_3(self.vertices[self.triangles[i].v0 as usize]),
            );
            node.aabb.max = crate::types::max(
                &node.aabb.max,
                &vec4_to_3(self.vertices[self.triangles[i].v1 as usize]),
            );
            node.aabb.max = crate::types::max(
                &node.aabb.max,
                &vec4_to_3(self.vertices[self.triangles[i].v2 as usize]),
            );
        }
    }

    fn find_split_axis(&self, node: &Node, centroids: &[Vec3]) -> (usize, f32, f32) {
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
                let cost = self.evaluate_sah(node, centroids, axis, candidate);
                if cost < best_cost {
                    best_position = candidate;
                    best_axis = axis;
                    best_cost = cost;
                }
            }
        }

        (best_axis, best_position, best_cost)
    }

    fn subdivide(&mut self, idx: usize, centroids: &mut [Vec3]) {
        let node = &self.nodes[idx];

        let (axis, split) = if !self.use_sah {
            let extent = node.aabb.extent();
            let axis = node.aabb.dominant_axis();
            let split = node.aabb.min[axis] + extent[axis] * 0.5;
            (axis, split)
        } else {
            let (axis, split, cost) = self.find_split_axis(node, centroids);
            let parent_area = node.aabb.area();
            let parent_cost = node.primitive_count as f32 * parent_area;
            // Only split if costs are higher or equal to parent
            if cost >= parent_cost {
                return;
            } else {
                (axis, split)
            }
        };

        let mut i = node.first_primitive as i64;
        let mut j = i + node.primitive_count as i64 - 1;
        while i <= j {
            if centroids[i as usize][axis] < split {
                i += 1;
            } else {
                self.triangles.swap(i as usize, j as usize);
                centroids.swap(i as usize, j as usize);
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

        self.update_bounds(left_child_index);
        self.update_bounds(right_child_index);
        self.subdivide(left_child_index, centroids);
        self.subdivide(right_child_index, centroids);
    }

    fn evaluate_sah(&self, node: &Node, centroids: &[Vec3], axis: usize, position: f32) -> f32 {
        let mut left_box = AABB::default();
        let mut right_box = AABB::default();

        let mut left_count = 0;
        let mut right_count = 0;

        let first = node.first_primitive as usize;
        let count = node.primitive_count as usize;

        for (i, centroid) in centroids.iter().enumerate().skip(first).take(count) {
            let triangle = &self.triangles[i];
            if centroid[axis] < position {
                left_count += 1;
                left_box.grow_with_position(&vec4_to_3(self.vertices[triangle.v0 as usize]));
                left_box.grow_with_position(&vec4_to_3(self.vertices[triangle.v1 as usize]));
                left_box.grow_with_position(&vec4_to_3(self.vertices[triangle.v2 as usize]));
            } else {
                right_count += 1;
                right_box.grow_with_position(&vec4_to_3(self.vertices[triangle.v0 as usize]));
                right_box.grow_with_position(&vec4_to_3(self.vertices[triangle.v1 as usize]));
                right_box.grow_with_position(&vec4_to_3(self.vertices[triangle.v2 as usize]));
            }
        }

        let cost = left_count as f32 * left_box.area() + right_count as f32 * right_box.area();
        if cost > 0.0 {
            cost
        } else {
            f32::MAX
        }
    }

    fn traverse_node_recursive(&self, idx: usize, ray: &Ray) {
        let node = &self.nodes[idx];
        let hit = intersect_aabb(&node.aabb, ray, f32::MAX) < f32::MAX;
        if hit {
            if node.primitive_count > 0 {
                let first = node.first_primitive as usize;
                let last = first + node.primitive_count as usize;
                for p in &self.triangles[first..last] {
                    intersect_triangle(
                        ray,
                        &self.vertices[p.v0 as usize],
                        &self.vertices[p.v1 as usize],
                        &self.vertices[p.v2 as usize],
                    );
                }
            } else {
                self.traverse_node_recursive(node.first_primitive as usize, ray);
                self.traverse_node_recursive(node.first_primitive as usize + 1, ray);
            }
        }
    }

    pub fn traverse_recursive(&self, ray: &Ray) {
        self.traverse_node_recursive(0, ray)
    }

    pub fn traverse_stack(&self, ray: &Ray) -> f32 {
        let mut node_idx = 0;
        let mut stack_ptr = 0;
        let mut stack = [0; 64];
        let mut d = f32::MAX;
        loop {
            let node = &self.nodes[node_idx];
            if self.nodes[node_idx].primitive_count > 0 {
                let first = node.first_primitive as usize;
                let last = first + node.primitive_count as usize;
                for p in &self.triangles[first..last] {
                    let distance = intersect_triangle(
                        ray,
                        &self.vertices[p.v0 as usize],
                        &self.vertices[p.v1 as usize],
                        &self.vertices[p.v2 as usize],
                    );
                    if distance < d {
                        d = distance;
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

        d
    }
}
