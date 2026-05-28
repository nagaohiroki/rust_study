struct Uniforms {
    time: f32,
};
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};
@group(0) @binding(0)
var<uniform> my_uniforms: Uniforms;
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = model.color;
    return out;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let plus = (sin(my_uniforms.time * 2.0) + 1.0) * 0.5;
    return vec4<f32>(in.color * plus, 1.0);
}
