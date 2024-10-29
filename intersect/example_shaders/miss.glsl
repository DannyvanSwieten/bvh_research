void first_miss_shader(Ray ray, inout RayPayload payload)
{
    // interpolate sky color based on y direction
    float d = 0.5 * (ray.direction.y + 1.0);
    payload.color *= (1.0 - d) * vec3(1.0, 1.0, 1.0) + d * vec3(0.5, 0.7, 1.0);
    payload.hit = false;
}