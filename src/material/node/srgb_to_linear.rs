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

pub struct SRGBToLinearNode {
	node: ResourceId<Box<dyn MaterialNode>>,
}

impl SRGBToLinearNode {
	pub fn new(
		node: ResourceId<Box<dyn MaterialNode>>,
	) -> Self {
		SRGBToLinearNode {
			node: node,
		}
	}
}

impl MaterialNode for SRGBToLinearNode {
	fn collect_nodes (
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<ResourceId<Box<dyn MaterialNode>>>,
		visited: &mut HashMap<ResourceId<Box<dyn MaterialNode>>, bool>,
		self_rid: ResourceId<Box<dyn MaterialNode>>,
	) {
		pool.borrow(&self.node).unwrap().collect_nodes(
			pool, nodes, visited, self.node,
		);
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
		pool: &ResourcePool<Box<dyn MaterialNode>>,
		visited: &mut HashMap<usize, bool>,
		self_id: usize,
	) -> String {
		if visited.contains_key(&self_id) {
			return "".to_string();
		}
		visited.insert(self_id, true);

		let node = pool.borrow(&self.node).unwrap();

		node.build_fragment_shader(pool, visited, self.node.id) +
		&format!("let {} = srgb_to_linear({});\n",
			self.get_fragment_output(self_id),
			node.get_fragment_output(self.node.id),
		)
	}

	fn get_fragment_output(&self, self_id: usize) -> String {
		format!("srgb_to_linear_output_{}", self_id)
	}
}
