#include "random.glsl"

layout(scalar, set = 1, binding = 1) readonly buffer VertexBuffer {
    vec3 vertices[];
};

layout(scalar, set = 1, binding = 0) readonly buffer IndexBuffer {
    uint indices[];
};

// Uses the inversion method to map two uniformly random numbers to a three dimensional
// unit hemisphere where the probability of a given sample is proportional to the cosine
// of the angle between the sample direction and the "up" direction (0, 1, 0)
vec3 sampleCosineWeightedHemisphere(vec2 u) {
    float phi = 2.0f * M_PI_F * u.x;
    
    float cos_phi = cos(phi);
    float sin_phi = sin(phi);
    
    float cos_theta = sqrt(u.y);
    float sin_theta = sqrt(1.0 - cos_theta * cos_theta);
    
    return vec3(sin_theta * cos_phi, cos_theta, sin_theta * sin_phi);
}

Ray shade(Ray ray, uint instance_id, uint primitive_id, float t, vec2 attributes, mat4 transform){
    if(t < FLOAT_MAX)
    {
        uint i0 = indices[primitive_id];
        uint i1 = indices[primitive_id + 1];
        uint i2 = indices[primitive_id + 2];

        vec3 v0 = vertices[i0];
        vec3 v1 = vertices[i1];
        vec3 v2 = vertices[i2];

        vec3 e0 = v2 - v0;
        vec3 e1 = v1 - v0;
        vec3 N = normalize(cross(e1, e0));
        vec3 L = vec3(1, 1, -1);
        L = normalize(L);
        float I = max(0, dot(N, L));
        return Ray(vec3(0.1, .2, .5), vec3(0.6, .7, 8), vec3(1) * ray.color + vec3(1) * I);
    } 
    else 
    {
        float d = 0.5 * (ray.direction.y + 1.0);
        vec3 color = (1.0 - d) * vec3(1.0, 1.0, 1.0) + d * vec3(0.5, 0.7, 1.0);
        return Ray(vec3(0.1, .2, .5), vec3(0.6, .7, 8), color);
    }
}