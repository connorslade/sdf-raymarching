@group(0) @binding(0) var<uniform> ctx: Uniform;

const ε: f32 = 0.01;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray = ray_direction(vec2(in.uv.x, 1 - in.uv.y) - 0.5);
    var pos = ctx.camera.pos;

    for (var i = 0; i < 100; i++) {
        let hit = field(pos);
        let dist = hit.dist;

        if dist < 0.0001 {
            // approx normal dir in world space
            let dx = dist - field(pos + vec3(ε, 0, 0)).dist;
            let dy = dist - field(pos + vec3(0, ε, 0)).dist;
            let dz = dist - field(pos + vec3(0, 0, ε)).dist;
            let normal = normalize(vec3(dx, dy, dz));

            // blinn phong lighting
            let light_dir = normalize(vec3f(-1));
            let diffuse = max(dot(normal, light_dir), 0.0);

            let reflect_dir = reflect(-light_dir, normal);
            let specular = pow(max(dot(light_dir, reflect_dir), 0.0), 32.0);

            let intensity = (diffuse + 0.1) * hit.mat.color + specular * hit.mat.highlight;

            return vec4(tone_map(intensity), 1.0);
        }

        pos += ray * dist;
    }

    return vec4(0.0);
}

// From https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
fn tone_map(x: vec3f) -> vec3f {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return saturate((x * (a * x + b)) / (x * (c * x + d) + e));
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
