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

pub struct ConstVector3Node {
	value: [f32; 3],
}

impl ConstVector3Node {
	pub fn new(value: [f32; 3]) -> Self {
		ConstVector3Node {
			value: value,
		}
	}
}

impl MaterialNode for ConstVector3Node {
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
		format!("vec3<f32>({}, {}, {})", self.value[0], self.value[1], self.value[2])
	}
}
