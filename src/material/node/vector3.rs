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

pub struct Vector3Node {
	contents: UniformContents,
}

impl Vector3Node {
	pub fn new(value: [f32; 3]) -> Self {
		Vector3Node {
			contents: UniformContents::Vector3 {
				value: value,
			},
		}
	}
}

impl MaterialNode for Vector3Node {
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
		Some(&self.contents)
	}

	fn build_declaration(&self, self_id: usize) -> String {
		format!("vector3_{}: vec3<f32>;\n", self_id)
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

	fn get_fragment_output(&self, self_id: usize) -> String {
		format!("unif.vector3_{}", self_id)
	}
}
