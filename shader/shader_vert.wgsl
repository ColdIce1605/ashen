[[stage(vertex)]]

fn main(
    [[location(0)]] a_position: vec3<f32>,
    [[location(1)]] a_color: vec3<f32>,
) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(a_position, 1.0);
}