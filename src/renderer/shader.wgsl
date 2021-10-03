struct VertexOutput {
	[[builtin(position)]] position: vec4<f32>;
	[[location(1)]] normal: vec3<f32>;
};

[[block]]
struct Object {
	modelViewMatrix: mat4x4<f32>;
};

[[block]]
struct Camera {
	projectionMatrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> object: Object;

[[group(0), binding(1)]]
var<uniform> camera: Camera;

[[stage(vertex)]]
fn vs_main(
	[[location(0)]] position: vec3<f32>,
	[[location(1)]] normal: vec3<f32>,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = camera.projectionMatrix * object.modelViewMatrix * vec4<f32>(position, 1.0);
	out.normal = normal;
	return out;
}

[[block]]
struct Material {
	color: vec3<f32>;
};

[[group(0), binding(2)]]
var<uniform> material: Material;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	return vec4<f32>(material.color, 1.0);
}
