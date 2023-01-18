use cgmath::InnerSpace;

use crate::types::{Ray, Vertex, AABB};

pub fn intersect_aabb(aabb: &AABB, ray: &Ray, t_far: f32) -> f32 {
    let tx1 = (aabb.min.x - ray.origin.x) / ray.direction.x;
    let tx2 = (aabb.max.x - ray.origin.x) / ray.direction.x;
    let ty1 = (aabb.min.y - ray.origin.y) / ray.direction.y;
    let ty2 = (aabb.max.y - ray.origin.y) / ray.direction.y;
    let tz1 = (aabb.min.z - ray.origin.z) / ray.direction.z;
    let tz2 = (aabb.max.z - ray.origin.z) / ray.direction.z;

    let t_min = tx1.min(tx2);
    let t_max = tx1.max(tx2);

    let t_min = t_min.max(ty1.min(ty2));
    let t_max = t_max.min(ty1.max(ty2));

    let t_min = t_min.max(tz1.min(tz2));
    let t_max = t_max.min(tz1.max(tz2));

    let hit = t_max >= t_min && t_min < t_far && t_max > 0.0;
    if hit {
        t_min.min(t_max)
    } else {
        t_far
    }
}

pub fn intersect_triangle(ray: &Ray, v0: &Vertex, v1: &Vertex, v2: &Vertex) -> f32 {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = ray.direction.cross(edge2);
    let a = edge1.dot(h);
    if a > -0.0001 && a < 0.0001 {
        // Ray is parallel to the triangle
        return f32::MAX;
    }

    let f = 1.0 / a;
    let s = ray.origin - v0;
    let u = f * s.dot(h);
    if !(0.0..=1.0).contains(&u) {
        return f32::MAX;
    }
    let q = s.cross(edge1);
    let v = f * ray.direction.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return f32::MAX;
    }

    let t = f * edge2.dot(q);
    if t < 0.000001 {
        return f32::MAX;
    }

    t
}
