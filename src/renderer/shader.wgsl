struct VertexOutput {
	[[builtin(position)]] position: vec4<f32>;
	[[location(1)]] normal: vec3<f32>;
};

[[block]]
struct Object {
	modelMatrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> object: Object;

[[stage(vertex)]]
fn vs_main(
	[[location(0)]] position: vec3<f32>,
	[[location(1)]] normal: vec3<f32>,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = object.modelMatrix * vec4<f32>(position, 1.0);
	out.normal = normal;
	return out;
}

[[block]]
struct Material {
	color: vec3<f32>;
};

[[group(0), binding(1)]]
var<uniform> material: Material;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	return vec4<f32>(material.color, 1.0);
}
