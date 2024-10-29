#import "random.glsl"

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
    return ray;
}

void ray_generation_shader(uvec2 pixel, ivec2 resolution, inout RayPayload payload) {
    // Apply a random offset to random number index to decorrelate pixel    
    int spp = 128;
    int max_depth = 6;
    float f = 1.0 / float(spp);
    vec3 color = vec3(0.0);
    uint seed = rand_seed(uint(pixel.x), uint(pixel.y));
    for(int i = 0; i < spp; ++i)
    {
        vec2 r = vec2(rand_float(seed), rand_float(seed));
        Ray ray = create_ray(resolution, pixel + r, vec3(-0.5, 0.0, -3.0), 3.0);
        payload.color = vec3(1.0);

        for(int depth = 0; depth < max_depth; ++depth)
        {
            trace(ray, 0.01, 1000.0, 0, 0, CULL_MASK_OPAQUE, payload);
            if(payload.hit)
            {
                vec2 u = vec2(rand_float(seed), rand_float(seed));
                sample_cosine_weighted_hemisphere(u);
                mat3 cs = create_coordinate_system(payload.normal);
                ray.direction = cs * sample_cosine_weighted_hemisphere(u);
                ray.origin = ray.origin + ray.direction * payload.t;
            }
            else
            {
                break;
            }
        }

        color += payload.color;
    }
    imageStore(result, ivec2(pixel), vec4(color * f, 1.0));
}