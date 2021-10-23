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

pub struct ConstFloatNode {
	value: f32,
}

impl ConstFloatNode {
	pub fn new(value: f32) -> Self {
		ConstFloatNode {
			value: value,
		}
	}
}

impl MaterialNode for ConstFloatNode {
	fn collect_nodes<'a> (
		&'a self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		_nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>,
	) {
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		None
	}

	fn build_declaration(&self) -> String {
		format!("")
	}

	fn build_functions(&self) -> String {
		format!("")
	}

	fn build_fragment_shader(
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		format!("")
	}

	fn get_fragment_output(&self) -> String {
		format!("{}", self.value)
	}
}
