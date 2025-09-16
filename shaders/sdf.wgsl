fn field(p: vec3f) -> f32 {
    return min(
        sdf_sphere(p, 1.0),
        sdf_box(p - vec3(2.0, 0.0, 0.0), vec3(1.0))
    );
}

// ↓ https://iquilezles.org/articles/distfunctions

fn sdf_sphere(p: vec3f, r: f32) -> f32 {
    return length(p) - 1.0;
}

fn sdf_box(p: vec3f, b: vec3f) -> f32 {
    let q = abs(p) - b;
    return length(max(q, vec3f(0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
}
