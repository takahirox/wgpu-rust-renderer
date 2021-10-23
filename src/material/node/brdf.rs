use crate::{
	material::node::node::{
		MaterialNode,
		UniformContents,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
};

const FUNCTION_CHUNK: &str = "
let PI: f32 = 3.1415926535;

fn d_ggx(n_dot_h: f32, roughness: f32) -> f32 {
  let a: f32 = n_dot_h * roughness;
  let k: f32 = roughness / (1.0 - pow(n_dot_h, 2.0) + pow(a, 2.0));
  return pow(k, 2.0) * (1.0 / PI);
}

fn v_smith_ggx_correlated_fast(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32 {
  let a: f32 = roughness;
  let ggxv: f32 = n_dot_l * (n_dot_v * (1.0 - a) + a);
  let ggxl: f32 = n_dot_v * (n_dot_l * (1.0 - a) + a);
  return 0.5 / (ggxv + ggxl);
}

fn brdf(
  v: vec3<f32>,
  n: vec3<f32>,
  h: vec3<f32>,
  l: vec3<f32>,
  base_color: vec3<f32>,
  metallic: f32,
  roughness: f32
) -> vec3<f32> {
  let black = vec3<f32>(0.0);
  let v_dot_h = dot(v, h);
  let n_dot_v = dot(v, n);
  let n_dot_l = dot(l, n);
  let n_dot_h = dot(n, h);

  let c_diff = mix(base_color, black, metallic);
  let f0 = mix(vec3<f32>(0.04), base_color, metallic);
  let alpha = pow(roughness, 2.0);

  let f = f0 + (1.0 - f0) * pow(1.0 - abs(v_dot_h), 5.0);

  let f_diffuse = (1.0 - f) * (1.0 / PI) * c_diff;
  let f_specular = f * d_ggx(n_dot_h, alpha)
    * v_smith_ggx_correlated_fast(n_dot_v, n_dot_l, alpha)
    / (4.0 * abs(n_dot_v) * abs(n_dot_l));

  return f_diffuse + f_specular;
}
";

pub struct BRDFNodeDescriptor {
	pub label: String,
	pub base_color: ResourceId<Box<dyn MaterialNode>>,
	pub metallic: ResourceId<Box<dyn MaterialNode>>,
	pub roughness: ResourceId<Box<dyn MaterialNode>>,
}

pub struct BRDFNode {
	desc: BRDFNodeDescriptor,
}

impl BRDFNode {
	pub fn new(
		desc: BRDFNodeDescriptor,
	) -> Self {
		BRDFNode {
			desc: desc,
		}
	}
}

impl MaterialNode for BRDFNode {
	fn collect_nodes<'a> (
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>,
	) {
		pool.borrow(&self.desc.base_color).unwrap().collect_nodes(pool, nodes);
		pool.borrow(&self.desc.metallic).unwrap().collect_nodes(pool, nodes);
		pool.borrow(&self.desc.roughness).unwrap().collect_nodes(pool, nodes);
		nodes.push(&self.desc.base_color);
		nodes.push(&self.desc.metallic);
		nodes.push(&self.desc.roughness);
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		None
	}

	fn build_declaration(&self) -> String {
		format!("")
	}

	fn build_functions(&self) -> String {
		FUNCTION_CHUNK.to_string()
	}

	fn build_fragment_shader(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		let base_color = pool.borrow(&self.desc.base_color).unwrap();
		let metallic = pool.borrow(&self.desc.metallic).unwrap();
		let roughness = pool.borrow(&self.desc.roughness).unwrap();

		base_color.build_fragment_shader(pool) +
		&metallic.build_fragment_shader(pool) +
		&roughness.build_fragment_shader(pool) +
		&format!("let brdf_v = normalize(in.view_dir);\n") +
		&format!("let brdf_l = normalize(light_dir);\n") +
		&format!("let brdf_n = normalize(in.normal);\n") +
		&format!("let brdf_h = normalize(brdf_l + brdf_v);\n") +
		&format!("let brdf_output = brdf(brdf_v, brdf_n, brdf_h, brdf_l, {}, {}, {});\n",
			base_color.get_fragment_output(),
			metallic.get_fragment_output(),
			roughness.get_fragment_output()
		) +
		// @TODO: Fix me
		&format!("use_directional_light = false;\n")
	}

	fn get_fragment_output(&self) -> String {
		format!("brdf_output")
	}
}
