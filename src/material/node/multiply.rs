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

pub struct MultiplyNode {
	value1: ResourceId<Box<dyn MaterialNode>>,
	value2: ResourceId<Box<dyn MaterialNode>>,
}

impl MultiplyNode {
	pub fn new(
		value1: ResourceId<Box<dyn MaterialNode>>,
		value2: ResourceId<Box<dyn MaterialNode>>,
	) -> Self {
		MultiplyNode {
			value1: value1,
			value2: value2,
		}
	}
}

impl MaterialNode for MultiplyNode {
	fn collect_nodes<'a> (
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>,
	) {
		pool.borrow(&self.value1).unwrap().collect_nodes(pool, nodes);
		pool.borrow(&self.value2).unwrap().collect_nodes(pool, nodes);
		nodes.push(&self.value1);
		nodes.push(&self.value2);
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
		let value1 = pool.borrow(&self.value1).unwrap();
		let value2 = pool.borrow(&self.value2).unwrap();

		value1.build_fragment_shader(pool) +
		&value2.build_fragment_shader(pool) +
		&format!("let multiply_output = {} * {};\n",
			value1.get_fragment_output(),
			value2.get_fragment_output())
	}

	fn get_fragment_output(&self) -> String {
		format!("multiply_output")
	}
}
