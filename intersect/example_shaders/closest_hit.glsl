
void closest_hit_shader(Intersection intersection, Ray ray, mat4 transform, inout RayPayload payload) {
        uint i0 = indices[intersection.primitive_id];
        uint i1 = indices[intersection.primitive_id + 1];
        uint i2 = indices[intersection.primitive_id + 2];

        vec3 v0 = (transform * vec4(vertices[i0], 1.0)).xyz;
        vec3 v1 = (transform * vec4(vertices[i1], 1.0)).xyz;
        vec3 v2 = (transform * vec4(vertices[i2], 1.0)).xyz;

        vec3 e0 = v2 - v0;
        vec3 e1 = v1 - v0;
        vec3 N = normalize(cross(e1, e0));
        payload.color = 0.5 * N + 0.5;
}