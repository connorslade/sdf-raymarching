fn field(p: vec3f) -> Object {
    let red = Material(vec3f(1, 0, 0), vec3f(1));
    let blue = Material(vec3f(0, 0, 1), vec3f(1));

    return sdf_smooth_union(
        Object(sdf_sphere(p, vec3f(2, 0, 0), 1.0), red),
        Object(sdf_box(p, vec3(0.0), vec3(1.0)), blue),
        0.1 + (1.0 + sin(ctx.t)) / 4
    );
}

fn sdf_mix(a: Object, b: Object, t: f32) -> Object {
    return Object(mix(a.dist, b.dist, t), Material(
        mix(a.mat.color, b.mat.color, t),
        mix(a.mat.highlight, b.mat.highlight, t)
    ));
}

fn sdf_union(a: Object, b: Object) -> Object {
    if a.dist < b.dist { return a; } else { return b; }
}

fn sdf_smooth_union(a: Object, b: Object, k: f32) -> Object {
    let r = exp(-a.dist / k) + exp(-b.dist / k);
    let dist = -k * log(r);

    let dmin = min(a.dist, b.dist);
    let dmax = max(a.dist, b.dist);

    // ↓ fix this
    let t = exp(-exp(-a.dist / k) / exp(-b.dist / k));
    let mat = Material(
        mix(a.mat.color, b.mat.color, t),
        mix(a.mat.highlight, b.mat.highlight, t)
    );

    return Object(dist, mat);
}

struct Material {
    color: vec3f,
    highlight: vec3f
}

struct Object {
    dist: f32,
    mat: Material
}

fn sdf_sphere(p: vec3f, c: vec3f, r: f32) -> f32 {
    return length(p - c) - 1.0;
}

// From https://iquilezles.org/articles/distfunctions
fn sdf_box(p: vec3f, c: vec3f, b: vec3f) -> f32 {
    let q = abs(p - c) - b;
    return length(max(q, vec3f(0.0))) + min(max(q.x,max(q.y,q.z)),0.0);
}
