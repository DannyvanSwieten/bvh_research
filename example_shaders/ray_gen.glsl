#include "random.glsl"

layout(push_constant) uniform FrameData {
    mat4 view_inverse;
    mat4 proj_inverse;
    uint current_sample;
    uint current_bounce;
};

Ray create_ray(vec2 resolution, vec2 frag_location){
    vec2 st = frag_location / resolution;
    st = st * 2.0 - 1.0;
    st.y = -st.y;

    vec3 pixel_position = (proj_inverse * vec4(st, 1, 1)).xyz;
    vec3 origin = (view_inverse * vec4(0, 0, 0, 1)).xyz;

    vec3 direction = (view_inverse * vec4(normalize(pixel_position - origin), 0)).xyz;
    Ray ray;
    ray.origin = origin;
    ray.direction = direction;
    ray.color = vec3(0);
    return ray;
}

Ray generate_ray(vec2 pixel, vec2 resolution){
    // Apply a random offset to random number index to decorrelate pixels
    uint offset = rand_seed(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);
    
    // Add a random offset to the pixel coordinates for antialiasing
    vec2 r = vec2(halton(offset + current_sample, 0),
                        halton(offset + current_sample, 1));
    
    return create_ray(resolution, pixel + r);
}