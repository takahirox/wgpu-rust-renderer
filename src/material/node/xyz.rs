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

pub struct XYZNode {
	node: ResourceId<Box<dyn MaterialNode>>,
}

impl XYZNode {
	pub fn new(
		node: ResourceId<Box<dyn MaterialNode>>,
	) -> Self {
		XYZNode {
			node: node,
		}
	}
}

impl MaterialNode for XYZNode {
	fn collect_nodes<'a> (
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>) {
		pool.borrow(&self.node).unwrap().collect_nodes(pool, nodes);
		nodes.push(&self.node);
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
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		let node = pool.borrow(&self.node).unwrap();
		node.build_fragment_shader(pool) +
		&format!("let xyz_output = {}.xyz;\n", node.get_fragment_output())
	}

	fn get_fragment_output(&self) -> String {
		format!("xyz_output")
	}
}
