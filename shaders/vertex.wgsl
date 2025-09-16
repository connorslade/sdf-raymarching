struct VertexInput {
    @location(0) pos: vec4<f32>,
    @location(1) uv: vec2<f32>
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(1) uv: vec2<f32>,
}
