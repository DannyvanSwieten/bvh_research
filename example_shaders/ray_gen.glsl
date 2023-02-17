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
    ray.color = vec3(0);
    return ray;
}

Ray generate_ray(vec2 pixel, vec2 resolution){
    return create_ray(resolution, pixel, vec3(-5.0, 0.0, -15.0), 2.0);
}