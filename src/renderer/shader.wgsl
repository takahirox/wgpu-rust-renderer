struct VertexOutput {
	[[builtin(position)]] position: vec4<f32>;
	[[location(1)]] normal: vec3<f32>;
};

[[block]]
struct Object {
	model_view_matrix: mat4x4<f32>;
	normal_matrix: mat3x3<f32>;
};

[[block]]
struct Camera {
	projection_matrix: mat4x4<f32>;
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
	out.position = camera.projection_matrix * object.model_view_matrix * vec4<f32>(position, 1.0);
	out.normal = normalize(object.normal_matrix * normal);
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
	var light_dir: vec3<f32> = normalize(vec3<f32>(0.0, 0.0, 1.0));
	var light_color: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
	var light_factor = clamp(dot(normalize(in.normal), light_dir), 0.0, 1.0) * light_color;
	return vec4<f32>(material.color * light_factor, 1.0);
}
