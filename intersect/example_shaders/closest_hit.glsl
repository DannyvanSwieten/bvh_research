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

uint rand_int(inout uint seed)
{
	// LCG values from Numerical Recipes
	return (seed = 1664525 * seed + 1013904223);
}

float rand_float(inout uint seed)
{
	// Float version using bitmask from Numerical Recipes
	const uint one = 0x3f800000;
	const uint msk = 0x007fffff;
	return uintBitsToFloat(one | (msk & (rand_int(seed) >> 9))) - 1;
}

void closest_hit_shader(Intersection intersection, Ray ray, mat4 transform){

        // uint i0 = indices[primitive_id];
        // uint i1 = indices[primitive_id + 1];
        // uint i2 = indices[primitive_id + 2];

        // vec3 v0 = (transform * vec4(vertices[i0], 1.0)).xyz;
        // vec3 v1 = (transform * vec4(vertices[i1], 1.0)).xyz;
        // vec3 v2 = (transform * vec4(vertices[i2], 1.0)).xyz;

        // vec3 e0 = v2 - v0;
        // vec3 e1 = v1 - v0;
        // vec3 N = normalize(cross(e1, e0));
        // vec3 L = vec3(1, 1, -1);
        // L = normalize(L); 
        // float I = max(0, dot(N, L));
        // vec3 color = ray.color.xyz + vec3(1);
        // color = vec3(0);
        // return Ray(vec3(0.1, .2, .5), vec3(0.6, .7, 8), vec4(color, 1.0));
    // } 
    // else 
    // {
    //     float d = 0.5 * (ray.direction.y + 1.0);
    //     vec3 color = (1.0 - d) * vec3(1.0, 1.0, 1.0) + d * vec3(0.5, 0.7, 1.0);
    //     return Ray(vec3(0.1, .2, .5), vec3(0.6, .7, 8), vec4(color, 1.0));
    // }
}