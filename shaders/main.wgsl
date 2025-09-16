@group(0) @binding(0) var<uniform> ctx: Uniform;

const PI: f32 = 3.1415926538;

@fragment
fn frag(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray = ray_direction(in.uv);
    var pos = ctx.camera.pos;

    for (var i = 0; i < 100; i++) {
        let val = field(pos);
        if val < 0.1 { return vec4(1.0); }

        pos += ray * 0.1;
    }

    return vec4(0.0);
}

fn camera_direction() -> vec3f {
    var pitch = ctx.camera.pitch;
    var yaw = -ctx.camera.yaw;

    return normalize(vec3(
        cos(yaw) * cos(pitch),
        sin(pitch),
        sin(yaw) * cos(pitch)
    ));
}

fn ray_direction(pos: vec2f) -> vec3f {
    let forward = camera_direction();
    let right = -normalize(cross(vec3f(0, 1, 0), forward));
    let up = -normalize(cross(forward, right));

    let fov_scale = tan(ctx.camera.fov * 0.5);
    let uv = pos * vec2(ctx.camera.aspect, 1.0) * fov_scale;

    return normalize(forward + right * uv.x + up * uv.y);
}
