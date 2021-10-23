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

pub struct Vector3Node {
	contents: UniformContents,
	label: String,
}

impl Vector3Node {
	pub fn new(label: &str, value: [f32; 3]) -> Self {
		Vector3Node {
			contents: UniformContents::Vector3 {
				value: value,
			},
			label: label.to_string(),
		}
	}

	fn get_name(&self) -> String {
		format!("vec3_{}", &self.label)
	}
}

impl MaterialNode for Vector3Node {
	fn collect_nodes<'a> (
		&'a self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		_nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>) {
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		Some(&self.contents)
	}

	fn build_declaration(&self) -> String {
		format!("{}: vec3<f32>;\n", self.get_name())
	}

	fn build_functions(&self) -> String {
		format!("")
	}

	fn build_fragment_shader(
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,	
	) -> String {
		format!("let {}_output = unif.{};\n", self.get_name(), self.get_name())
	}

	fn get_fragment_output(&self) -> String {
		format!("{}_output", self.get_name())
	}
}
