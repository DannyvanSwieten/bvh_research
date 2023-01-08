use crate::types::{Triangle, Vec3, Vertex, AABB};

struct Node {
    pub first_primitive: u32,
    pub primitive_count: u32,
    pub aabb: AABB,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            first_primitive: 0,
            primitive_count: 0,
            aabb: AABB::default(),
        }
    }
}
pub struct BVHMidPointSplit {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
    nodes: Vec<Node>,
    used_nodes: usize,
}

impl BVHMidPointSplit {
    pub fn new(vertices: &[Vertex], triangles: &[Triangle]) -> Self {
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
                (vertices[triangle.v0 as usize]
                    + vertices[triangle.v1 as usize]
                    + vertices[triangle.v2 as usize])
                    / 3.0
            })
            .collect();

        let mut this = Self {
            nodes,
            vertices: vertices.to_vec(),
            triangles: triangles.to_vec(),
            used_nodes: 1,
        };
        this.update_bounds(0);
        this.subdivide(0, &mut centroids);
        this
    }

    fn update_bounds(&mut self, idx: usize) {
        let mut node = &mut self.nodes[idx];
        let first = node.first_primitive as usize;
        let last = first + node.primitive_count as usize;
        for i in first..last as usize {
            node.aabb.min = crate::types::min(
                &node.aabb.min,
                &self.vertices[self.triangles[i].v0 as usize],
            );
            node.aabb.min = crate::types::min(
                &node.aabb.min,
                &self.vertices[self.triangles[i].v1 as usize],
            );
            node.aabb.min = crate::types::min(
                &node.aabb.min,
                &self.vertices[self.triangles[i].v2 as usize],
            );

            node.aabb.max = crate::types::max(
                &node.aabb.max,
                &self.vertices[self.triangles[i].v0 as usize],
            );
            node.aabb.max = crate::types::max(
                &node.aabb.max,
                &self.vertices[self.triangles[i].v1 as usize],
            );
            node.aabb.max = crate::types::max(
                &node.aabb.max,
                &self.vertices[self.triangles[i].v2 as usize],
            );
        }
    }

    fn subdivide(&mut self, idx: usize, centroids: &mut [Vec3]) {
        if self.nodes[idx].primitive_count < 2 {
            return;
        }
        let extent = self.nodes[idx].aabb.extent();
        let axis = self.nodes[idx].aabb.dominant_axis();
        let split = self.nodes[idx].aabb.min[axis] + extent[axis] * 0.5;

        let mut i = self.nodes[idx].first_primitive as i64;
        let mut j = i + self.nodes[idx].primitive_count as i64 - 1;
        while i <= j {
            if centroids[i as usize][axis] < split {
                i += 1;
            } else {
                self.triangles.swap(i as usize, j as usize);
                centroids.swap(i as usize, j as usize);
                j -= 1;
            }
        }

        let left_count = i as usize - self.nodes[idx].first_primitive as usize;
        if left_count == 0 || left_count == self.nodes[idx].primitive_count as usize {
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
}
