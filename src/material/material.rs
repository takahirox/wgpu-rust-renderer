use std::collections::HashMap;
use crate::{
	material::node::node::{
		MaterialNode,
		UniformContents,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
	texture::{
		sampler::Sampler,
		texture::Texture,
	},
};

pub enum Side {
	BackSide,
	DoubleSide,
	FrontSide,
}

impl Default for Side {
	fn default() -> Self {
		Side::FrontSide
	}
}

const PREFIX_CHUNK1: &str = "struct VertexOutput {
  [[builtin(position)]] position: vec4<f32>;
  [[location(1)]] normal: vec3<f32>;
  [[location(2)]] uv: vec2<f32>;
  [[location(3)]] view_position: vec3<f32>;
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

const PREFIX_CHUNK3: &str = "
let PI: f32 = 3.1415926535;

fn less_than_equal_f32(value1: f32, value2: f32) -> f32 {
  if (value1 <= value2) {
    return 1.0;
  }
  return 0.0;
}

fn less_than_equal_vec3_f32(value1: vec3<f32>, value2: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(
    less_than_equal_f32(value1.x, value2.x),
    less_than_equal_f32(value1.y, value2.y),
    less_than_equal_f32(value1.z, value2.z)
  );
}

fn srgb_to_linear(value: vec4<f32>) -> vec4<f32> {
  return vec4<f32>(
    mix(
      pow(value.rgb * 0.9478672986 + vec3<f32>(0.0521327014), vec3<f32>(2.4)),
      value.rgb * 0.0773993808,
      less_than_equal_vec3_f32(value.rgb, vec3<f32>(0.04045))
    ),
    value.a
  );
}

fn linear_to_srgb(value: vec4<f32>) -> vec4<f32> {
  return vec4<f32>(
    mix(
      pow(value.rgb, vec3<f32>(0.41666)) * 1.055 - vec3<f32>(0.055),
      value.rgb * 12.92,
      vec3<f32>(less_than_equal_vec3_f32(value.rgb, vec3<f32>(0.0031308)))
    ),
    value.a
  );
}
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
  out.view_position = -mv_position.xyz;
  return out;
}
";

const FRAGMENT_CHUNK1: &str = "
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
  var alpha = f32(1.0);
  let light_dir: vec3<f32> = normalize(vec3<f32>(0.0, 0.0, 1.0));
  // @TODO: Fix me
  var use_directional_light = true;
";

const FRAGMENT_CHUNK2: &str = "
  if (use_directional_light) {
    let light_color: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    let light_factor = clamp(dot(normalize(in.normal), light_dir), 0.0, 1.0) * light_color;
    color = color * light_factor.rgb;
  }
  // @TODO: Use material node?
  // @TODO: Color management
  return linear_to_srgb(vec4<f32>(color, alpha));
}
";

pub struct Material {
	color: ResourceId<Box<dyn MaterialNode>>,
	side: Side,
}

impl Material {
	pub fn new(
		color: ResourceId<Box<dyn MaterialNode>>,
		side: Side,
	) -> Self {
		Material {
			color: color,
			side: side,
		}
	}

	pub fn borrow_side(&self) -> &Side {
		&self.side
	}

	// @TODO: Optimize?
	fn borrow_nodes(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> Vec<ResourceId<Box<dyn MaterialNode>>> {
		let mut nodes = Vec::new();
		let mut visited = HashMap::new();
		pool.borrow(&self.color).unwrap().collect_nodes(
			pool,
			&mut nodes,
			&mut visited,
			self.color,
		);
		nodes
	}

	// @TODO: Optimize?
	pub fn borrow_contents<'a>(
		&self,
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
		let mut founds = HashMap::new();

		for contents in self.borrow_contents(pool).iter() {
			match contents {
				UniformContents::Texture {texture, ..} => {
					if !founds.contains_key(texture) {
						textures.push(texture);
						founds.insert(*texture, true);
					}
				},
				_ => {},
			};
		}
		textures
	}

	// @TODO: Optimize?
	pub fn borrow_samplers<'a>(
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
	) -> Vec<&'a ResourceId<Sampler>> {
		let mut samplers = Vec::new();
		let mut founds = HashMap::new();

		for contents in self.borrow_contents(pool).iter() {
			match contents {
				UniformContents::Texture {sampler, ..} => {
					if !founds.contains_key(sampler) {
						samplers.push(sampler);
						founds.insert(*sampler, true);
					}
				},
				_ => {},
			};
		}
		samplers
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
		PREFIX_CHUNK3 +
		&self.build_functions(pool)
	}

	fn build_vertex_shader(&self) -> String {
		VERTEX_CHUNK.to_string()
	}

	fn build_fragment_shader(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		let mut visited = HashMap::new();
		let color = pool.borrow(&self.color).unwrap();

		FRAGMENT_CHUNK1.to_string() +
		&color.build_fragment_shader(pool, &mut visited, self.color.id) +
		&format!("var color: vec3<f32> = {};\n", color.get_fragment_output(self.color.id)) +
		&FRAGMENT_CHUNK2.to_string()
	}

	// @TODO: Optimize?
	fn build_uniform_block_declaration(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		// bindings for textures start with 3
		let mut s = "".to_string();
		for node_id in self.borrow_nodes(pool).iter() {
			let node = pool.borrow(node_id).unwrap();
			if let Some(contents) = node.borrow_contents() {
				match contents {
					UniformContents::Texture {..} => {},
					_ => {
						s += &node.build_declaration(node_id.id);
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

		// Textures first
		for texture in self.borrow_textures(pool).iter() {
			s += &format!("\n[[group(0), binding({})]]\n", binding);
			s += &format!("var texture_{}: texture_2d<f32>;\n", texture.id);
			binding += 1;
		}

		// Samplers next
		for sampler in self.borrow_samplers(pool).iter() {
			s += &format!("\n[[group(0), binding({})]]\n", binding);
			s += &format!("var sampler_{}: sampler;\n", sampler.id);
			binding += 1;
		}
		s
	}

	fn build_functions(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		let mut s = "".to_string();
		for node_id in self.borrow_nodes(pool).iter() {
			s += &pool.borrow(node_id).unwrap().build_functions(node_id.id);
		}
		s
	}
}
