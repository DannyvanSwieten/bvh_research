layout(scalar, set = 1, binding = 1) readonly buffer VertexBuffer {
    vec3 vertices[];
};

layout(scalar, set = 1, binding = 0) readonly buffer IndexBuffer {
    uint indices[];
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

// Generates a seed for a random number generator from 2 inputs plus a backoff
// https://github.com/nvpro-samples/optix_prime_baking/blob/master/random.h
// https://en.wikipedia.org/wiki/Tiny_Encryption_Algorithm
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

Ray shade(Ray ray, uint instance_id, uint primitive_id, float t, vec2 attributes, mat4 transform){
    if(t < 1000 && t > 0)
    {
        uint i0 = indices[primitive_id];
        uint i1 = indices[primitive_id + 1];
        uint i2 = indices[primitive_id + 2];

        vec3 v0 = (transform * vec4(vertices[i0], 1.0)).xyz;
        vec3 v1 = (transform * vec4(vertices[i1], 1.0)).xyz;
        vec3 v2 = (transform * vec4(vertices[i2], 1.0)).xyz;

        vec3 e0 = v2 - v0;
        vec3 e1 = v1 - v0;
        vec3 N = normalize(cross(e1, e0));
        vec3 L = vec3(1, 1, -1);
        L = normalize(L);
        float I = max(0, dot(N, L));
        vec3 color = ray.color.xyz + vec3(1) * I;
        return Ray(vec3(0.1, .2, .5), vec3(0.6, .7, 8), vec4(color, 1.0));
    } 
    else 
    {
        float d = 0.5 * (ray.direction.y + 1.0);
        vec3 color = (1.0 - d) * vec3(1.0, 1.0, 1.0) + d * vec3(0.5, 0.7, 1.0);
        return Ray(vec3(0.1, .2, .5), vec3(0.6, .7, 8), vec4(color, 1.0));
    }
}