struct Uniform {
    window: vec2<u32>,
    steps: u32,

    camera: Camera,
    t: f32
}

struct Camera {
    pos: vec3f,
    pitch: f32,
    yaw: f32,

    fov: f32,
    aspect: f32,
}

@vertex
fn vert(in: VertexInput) -> VertexOutput {
    return VertexOutput(in.pos, in.uv);
}
