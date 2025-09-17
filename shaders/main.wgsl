@group(0) @binding(0) var<uniform> ctx: Uniform;

const ε: f32 = 0.01;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray = ray_direction(vec2(in.uv.x, 1 - in.uv.y) - 0.5);
    var pos = ctx.camera.pos;

    for (var i = 0; i < 100; i++) {
        let dist = field(pos);
        if dist < 0.0001 {
            // approx normal dir in world space
            let dx = dist - field(pos + vec3(ε, 0, 0));
            let dy = dist - field(pos + vec3(0, ε, 0));
            let dz = dist - field(pos + vec3(0, 0, ε));
            let normal = normalize(vec3(dx, dy, dz));

            // blinn phong lighting
            let light_dir = normalize(vec3f(-1));
            let diffuse = max(dot(normal, light_dir), 0.0);

            let reflect_dir = reflect(-light_dir, normal);
            let specular = pow(max(dot(light_dir, reflect_dir), 0.0), 32.0);

            let color = vec3(1.0);
            let intensity = (diffuse + specular + 0.1) * color;

            return vec4(intensity, 1.0);
        }

        pos += ray * dist;
    }

    return vec4(0.0);
}

fn camera_direction() -> vec3f {
    var pitch = ctx.camera.pitch;
    var yaw = ctx.camera.yaw;

    return normalize(vec3(
        sin(yaw) * cos(pitch),
        sin(pitch),
        cos(yaw) * cos(pitch)
    ));
}

fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = -normalize(cross(vec3f(0, 1, 0), forward));
    let up = normalize(cross(forward, right));

    let fov_scale = tan(ctx.camera.fov * 0.5);
    let uv = pos * vec2(ctx.camera.aspect, 1.0) * fov_scale;

    return normalize(forward + right * uv.x + up * uv.y);
}
