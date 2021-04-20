struct VertexInput {
    [[location(0)]] a_position: vec3<f32>;
    [[location(1)]] a_color: vec3<f32>;
};

struct VertexOutput {
    [[location(0)]] v_color: vec3<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.v_color = in.a_color;
    out.position = vec4<f32>(in.a_position, 1.0);
    return out;
}