fn field(p: vec3f) -> f32 {
    return min(
        mix(
            sdf_sphere(p, vec3f(0.0), 1.0),
            sdf_box(p, vec3(0.0), vec3(1.0)),
            (sin(ctx.t) + 1.0) / 2.0
        ),
        sdf_ground(p)
    );
}

// ↓ https://iquilezles.org/articles/distfunctions

fn sdf_ground(p: vec3f) -> f32 {
    return p.y + 10.0;
}

fn sdf_sphere(p: vec3f, c: vec3f, r: f32) -> f32 {
    return length(p - c) - 1.0;
}

fn sdf_box(p: vec3f, c: vec3f, b: vec3f) -> f32 {
    let q = abs(p - c) - b;
    return length(max(q, vec3f(0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
}
