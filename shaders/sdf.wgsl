fn field(p: vec3f) -> f32 {
    return sdf_union(
        sdf_sphere(p, vec3f(1.0), 1.0),
        sdf_box(p, vec3(0.0), vec3(1.0))
    );
}

fn sdf_union(a: f32, b: f32) -> f32 {
    return min(a, b);
}

// struct Material {}

// ↓ https://iquilezles.org/articles/distfunctions

fn sdf_sphere(p: vec3f, c: vec3f, r: f32) -> f32 {
    return length(p - c) - 1.0;
}

fn sdf_box(p: vec3f, c: vec3f, b: vec3f) -> f32 {
    let q = abs(p - c) - b;
    return length(max(q, vec3f(0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
}
