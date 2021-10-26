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

pub struct AddNode {
	value1: ResourceId<Box<dyn MaterialNode>>,
	value2: ResourceId<Box<dyn MaterialNode>>,
}

impl AddNode {
	pub fn new(
		value1: ResourceId<Box<dyn MaterialNode>>,
		value2: ResourceId<Box<dyn MaterialNode>>,
	) -> Self {
		AddNode {
			value1: value1,
			value2: value2,
		}
	}
}

impl MaterialNode for AddNode {
	fn collect_nodes (
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<ResourceId<Box<dyn MaterialNode>>>,
		visited: &mut HashMap<ResourceId<Box<dyn MaterialNode>>, bool>,
		self_rid: ResourceId<Box<dyn MaterialNode>>,
	) {
		pool.borrow(&self.value1).unwrap().collect_nodes(
			pool, nodes, visited, self.value1,
		);
		pool.borrow(&self.value2).unwrap().collect_nodes(
			pool, nodes, visited, self.value2,
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

		let value1 = pool.borrow(&self.value1).unwrap();
		let value2 = pool.borrow(&self.value2).unwrap();

		value1.build_fragment_shader(pool, visited, self.value1.id) +
		&value2.build_fragment_shader(pool, visited, self.value2.id) +
		&format!("let {} = {} + {};\n",
			self.get_fragment_output(self_id),
			value1.get_fragment_output(self.value1.id),
			value2.get_fragment_output(self.value2.id),
		)
	}

	fn get_fragment_output(&self, self_id: usize) -> String {
		format!("add_output_{}", self_id)
	}
}
