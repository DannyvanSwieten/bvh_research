layout(set = 1, binding = 0) readonly buffer VertexBuffer {
    vec4 vertices;
};

layout(set = 1, binding = 1) readonly buffer IndexBuffer {
    uint indices;
};

Ray shade(uint instance_id, uint primitive_id, float t, vec2 attributes, mat4 transform){
    return Ray(vec3(0.1, .2, .5), vec3(0.6, .7, 8), vec3(1 - attributes.x - attributes.y, attributes.x, attributes.y) * float(t < FLOAT_MAX));
}