use crate::{
	material::node::node::{
		MaterialNode,
		UniformContents,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
	texture::texture::Texture,
};

const PREFIX_CHUNK1: &str = "struct VertexOutput {
  [[builtin(position)]] position: vec4<f32>;
  [[location(1)]] normal: vec3<f32>;
  [[location(2)]] uv: vec2<f32>;
  [[location(3)]] view_dir: vec3<f32>;
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

[[block]]
struct Uniform {
";

const PREFIX_CHUNK2: &str = "};

[[group(0), binding(0)]]
var<uniform> object: Object;

[[group(0), binding(1)]]
var<uniform> camera: Camera;

[[group(0), binding(2)]]
var<uniform> unif: Uniform;
";

const VERTEX_CHUNK: &str = "
[[stage(vertex)]]
fn vs_main(
  [[location(0)]] position: vec3<f32>,
  [[location(1)]] normal: vec3<f32>,
  [[location(2)]] uv: vec2<f32>,
) -> VertexOutput {
  var out: VertexOutput;
  let mv_position = object.model_view_matrix * vec4<f32>(position, 1.0);
  out.position = camera.projection_matrix * mv_position;
  out.normal = normalize(object.normal_matrix * normal);
  out.uv = uv;
  out.view_dir = normalize(-mv_position.xyz);
  return out;
}
";

const FRAGMENT_CHUNK1: &str = "
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
  var alpha = f32(1.0);
  let light_dir: vec3<f32> = normalize(vec3<f32>(0.0, 0.0, 1.0));
  let light_color: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
  let light_factor = clamp(dot(normalize(in.normal), light_dir), 0.0, 1.0) * light_color;
  // @TODO: Fix me
  var use_directional_light = true;
";

const FRAGMENT_CHUNK2: &str = "
  if (use_directional_light) {
    color = color * light_factor.rgb;
  }
  return vec4<f32>(color, alpha);
}
";

pub struct Material {
	color: ResourceId<Box<dyn MaterialNode>>
}

impl Material {
	pub fn new(color: ResourceId<Box<dyn MaterialNode>>) -> Self {
		Material {
			color: color,
		}
	}

	// @TODO: Optimize?
	fn borrow_nodes<'a>(
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
	) -> Vec<&'a ResourceId<Box<dyn MaterialNode>>> {
		let mut nodes = Vec::new();
		pool.borrow(&self.color).unwrap().collect_nodes(pool, &mut nodes);
		nodes.push(&self.color);
		nodes
	}

	// @TODO: Optimize?
	pub fn borrow_contents<'a>(
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
	) -> Vec<&'a UniformContents> {
		let mut contents = Vec::<>::new();
		for node in self.borrow_nodes(pool).iter() {
			let node = pool.borrow(node).unwrap();
			if let Some(c) = node.borrow_contents() {
				contents.push(c);
			}
		}
		contents
	}

	// @TODO: Optimize?
	pub fn borrow_textures<'a>(
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
	) -> Vec<&'a ResourceId<Texture>> {
		let mut textures = Vec::new();
		for contents in self.borrow_contents(pool).iter() {
			match contents {
				UniformContents::Texture {value} => {
					textures.push(value);
				},
				_ => {},
			};
		}
		textures
	}

	pub fn build_shader_code(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		self.build_prefix(pool) +
		&self.build_vertex_shader() +
		&self.build_fragment_shader(pool)
	}

	fn build_prefix(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		PREFIX_CHUNK1.to_string() +
		&self.build_uniform_block_declaration(pool) +
		PREFIX_CHUNK2 +
		&self.build_texture_declaration(pool) +
		&self.build_functions(pool)
	}

	fn build_vertex_shader(&self) -> String {
		VERTEX_CHUNK.to_string()
	}

	fn build_fragment_shader(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		let color = pool.borrow(&self.color).unwrap();

		FRAGMENT_CHUNK1.to_string() +
		&color.build_fragment_shader(pool) +
		&format!("var color: vec3<f32> = {};\n", color.get_fragment_output()) +
		&FRAGMENT_CHUNK2.to_string()
	}

	// @TODO: Optimize?
	fn build_uniform_block_declaration(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		// bindings for textures start with 3
		let mut s = "".to_string();
		for node in self.borrow_nodes(pool).iter() {
			let node = pool.borrow(node).unwrap();
			if let Some(contents) = node.borrow_contents() {
				match contents {
					UniformContents::Texture {value: _} => {},
					_ => {
						s += &node.build_declaration();
					},
				}
			}
		}
		s
	}

	// @TODO: Optimize?
	fn build_texture_declaration(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		// bindings for textures start with 3
		let mut binding = 3;
		let mut s = "".to_string();
		for node in self.borrow_nodes(pool).iter() {
			let node = pool.borrow(node).unwrap();
			if let Some(contents) = node.borrow_contents() {
				match contents {
					UniformContents::Texture {value: _} => {
						s += &format!("\n[[group(0), binding({})]]\n", binding);
						s += &format!("{};\n", node.build_declaration());
						binding += 1;
					},
					_ => {},
				}
			}
		}
		s
	}

	fn build_functions(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		let mut s = "".to_string();
		for node in self.borrow_nodes(pool).iter() {
			s += &pool.borrow(node).unwrap().build_functions();
		}
		s
	}
}
