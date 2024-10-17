layout(push_constant) uniform FrameData {
    uint current_sample;
    uint current_bounce;
};

#define M_PI_F 3.14159

const uint primes[] = {
    2,   3,  5,  7,
    11, 13, 17, 19,
    23, 29, 31, 37,
    41, 43, 47, 53,
};

// Returns the i'th element of the Halton sequence using the d'th prime number as a
// base. The Halton sequence is a "low discrepency" sequence: the values appear
// random but are more evenly distributed then a purely random sequence. Each random
// value used to render the image should use a different independent dimension 'd',
// and each sample (frame) should use a different index 'i'. To decorrelate each
// pixel, a random offset can be applied to 'i'.
float halton(uint i, uint d) {
    uint b = primes[d];
    
    float f = 1.0f;
    float invB = 1.0f / b;
    
    float r = 0;
    
    while (i > 0) {
        f = f * invB;
        r = r + f * (i % b);
        i = i / b;
    }
    
    return r;
}

uint rand_seed(uint val0, uint val1)
{
	uint v0 = val0, v1 = val1, s0 = 0;

	for (uint n = 0; n < 16; n++)
	{
		s0 += 0x9e3779b9;
		v0 += ((v1 << 4) + 0xa341316c) ^ (v1 + s0) ^ ((v1 >> 5) + 0xc8013ea4);
		v1 += ((v0 << 4) + 0xad90777d) ^ (v0 + s0) ^ ((v0 >> 5) + 0x7e95761e);
	}

	return v0;
}

Ray create_ray(vec2 resolution, vec2 frag_location, vec3 origin, float z){
    vec2 norm = frag_location / resolution;
    vec3 p0 = vec3(-1, 1, z);
    vec3 p1 = vec3(1, 1, z);
    vec3 p2 = vec3(-1, -1, z);

    vec3 pixel_position = 
            p0
            + (p1 - p0) * norm.x
            + (p2 - p0) * norm.y;

    vec3 direction = normalize(pixel_position - origin);
    Ray ray;
    ray.origin = origin;
    ray.direction = direction;
    ray.color = vec4(0);
    return ray;
}

Ray generate_ray(vec2 pixel, vec2 resolution){
    // Apply a random offset to random number index to decorrelate pixels
    uint offset = rand_seed(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);
    
    // Add a random offset to the pixel coordinates for antialiasing
    vec2 r = vec2(halton(offset + current_sample, 0),
                        halton(offset + current_sample, 1));
    
    return create_ray(resolution, pixel + r, vec3(-0.5, 0.0, -3.0), 3.0);
}