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
	fn collect_nodes (
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<ResourceId<Box<dyn MaterialNode>>>,
		visited: &mut HashMap<ResourceId<Box<dyn MaterialNode>>, bool>,
		self_rid: ResourceId<Box<dyn MaterialNode>>,
	) {
		if !visited.contains_key(&self_rid) {
			visited.insert(self_rid, true);
			nodes.push(self_rid);
		}
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		None
	}

	fn build_declaration(&self, _self_id: usize) -> String {
		format!("")
	}

	fn build_functions(&self, _self_id: usize) -> String {
		format!("")
	}

	fn build_fragment_shader(
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		_visited: &mut HashMap<usize, bool>,
		_self_id: usize,
	) -> String {
		format!("")
	}

	fn get_fragment_output(&self, _self_id: usize) -> String {
		format!("vec3<f32>({}, {}, {})",
			self.value[0],
			self.value[1],
			self.value[2],
		)
	}
}
