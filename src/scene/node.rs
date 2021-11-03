use crate::{
	math::{
		euler::Euler,
		matrix4::Matrix4,
		quaternion::Quaternion,
		vector3::Vector3,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
};

pub struct Node {
	children: Vec<ResourceId<Node>>,
	matrix: [f32; 16],
	parent: Option<ResourceId<Node>>,
	position: [f32; 3],
	quaternion: [f32; 4],
	rotation: [f32; 3],
	scale: [f32; 3],
	world_matrix: [f32; 16],
}

impl Node {
	pub fn new() -> Self {
		Node {
			children: Vec::new(),
			matrix: Matrix4::create(),
			parent: None,
			position: Vector3::create(),
			quaternion: Quaternion::create(),
			rotation: Euler::create(),
			scale: *Vector3::set(&mut Vector3::create(), 1.0, 1.0, 1.0),
			world_matrix: Matrix4::create(),
		}
	}

	pub fn borrow_parent(&self) -> Option<&ResourceId<Node>> {
		self.parent.as_ref()
	}

	pub fn borrow_children(&self) -> &Vec<ResourceId<Node>> {
		&self.children
	}

	pub fn borrow_position(&self) -> &[f32; 3] {
		&self.position
	}

	pub fn borrow_position_mut(&mut self) -> &mut [f32; 3] {
		&mut self.position
	}

	pub fn borrow_rotation(&self) -> &[f32; 3] {
		&self.rotation
	}

	pub fn borrow_rotation_mut(&mut self) -> &mut [f32; 3] {
		&mut self.rotation
	}

	pub fn borrow_scale(&self) -> &[f32; 3] {
		&self.scale
	}

	pub fn borrow_scale_mut(&mut self) -> &mut [f32; 3] {
		&mut self.scale
	}

	pub fn borrow_matrix(&self) -> &[f32; 16] {
		&self.matrix
	}

	pub fn borrow_world_matrix(&self) -> &[f32; 16] {
		&self.world_matrix
	}

	pub fn set_matrix(&mut self, matrix: &[f32; 16]) -> &mut Self {
		Matrix4::copy(&mut self.matrix, matrix);
		Matrix4::decompose(&mut self.position, &mut self.quaternion, &mut self.scale, &self.matrix);
		Euler::set_from_quaternion(&mut self.rotation, &self.quaternion);
		self
	}

	pub fn set_world_matrix(&mut self, matrix: &[f32; 16]) -> &mut Self {
		Matrix4::copy(&mut self.world_matrix, matrix);
		self
	}

	pub fn update_matrix(&mut self) -> &mut Self {
		Quaternion::set_from_euler(&mut self.quaternion, &self.rotation);
		Matrix4::compose(&mut self.matrix, &self.position, &self.quaternion, &self.scale);
		self
	}

	// @TODO: Optimize
	pub fn update_matrices(
		&mut self,
		pool: &mut ResourcePool<Node>,
	) {
		self.update_matrix();

		if let Some(parent) = self.borrow_parent() {
			let parent_matrix = pool.borrow(parent).unwrap().borrow_world_matrix();
			Matrix4::multiply(&mut self.world_matrix, parent_matrix, &self.matrix);
		} else {
			Matrix4::copy(&mut self.world_matrix, &self.matrix);
		}

		let mut stack = Vec::new();

		for child in self.children.iter() {
			stack.push(*child);
		}

		while let Some(rid) = stack.pop() {
			let parent_matrix = {
				let mut matrix = Matrix4::create();
				let node = pool.borrow_mut(&rid).unwrap();
				let parent = node.borrow_parent().cloned().unwrap();
				Matrix4::copy(&mut matrix, pool.borrow(&parent).unwrap().borrow_world_matrix());
				matrix
			};

			let node = pool.borrow_mut(&rid).unwrap();
			let mut matrix = Matrix4::create();
			Matrix4::multiply(&mut matrix, &parent_matrix, &node.borrow_matrix());
			node.set_world_matrix(&matrix);

			for child in node.children.iter() {
				stack.push(*child);
			}
		}
	}
}

pub struct NodeExecutor {
}

impl NodeExecutor {
	pub fn update_matrices(
		pool: &mut ResourcePool<Node>,
		root: &ResourceId<Node>,
	) {
		let mut stack = Vec::new();
		stack.push(*root);

		while let Some(rid) = stack.pop() {
			let node = pool.borrow_mut(&rid).unwrap();
			node.update_matrix();

			let parent_matrix = {
				let mut matrix = Matrix4::create();
				let node = pool.borrow_mut(&rid).unwrap();
				if let Some(parent) = node.borrow_parent().cloned() {
					Matrix4::copy(&mut matrix, pool.borrow(&parent).unwrap().borrow_world_matrix());
				}
				matrix
			};

			let node = pool.borrow_mut(&rid).unwrap();
			let mut matrix = Matrix4::create();
			Matrix4::multiply(&mut matrix, &parent_matrix, &node.borrow_matrix());
			node.set_world_matrix(&matrix);

			for child in node.children.iter() {
				stack.push(*child);
			}
		}
	}

	pub fn collect_nodes(
		pool: &ResourcePool<Node>,
		root: &ResourceId<Node>,
		nodes: &mut Vec<ResourceId<Node>>,
	) {
		let mut stack = Vec::new();
		stack.push(*root);
		nodes.push(*root);

		while let Some(rid) = stack.pop() {
			let node = pool.borrow(&rid).unwrap();
			for child in node.children.iter() {
				stack.push(*child);
				nodes.push(*child);
			}
		}
	}
}
