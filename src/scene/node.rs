use crate::math::{
	euler::Euler,
	matrix4::Matrix4,
	quaternion::Quaternion,
	vector3::Vector3,
};

pub struct Node {
	children_ids: Vec<usize>,
	matrix: [f32; 16],
	parent_id: Option<usize>, 
	position: [f32; 3],
	quaternion: [f32; 4],
	rotation: [f32; 3],
	scale: [f32; 3],
}

impl Node {
	pub fn new() -> Self {
		Node {
			children_ids: Vec::new(),
			matrix: Matrix4::create(),
			parent_id: None,
			position: Vector3::create(),
			quaternion: Quaternion::create(),
			rotation: Euler::create(),
			scale: *Vector3::set(&mut Vector3::create(), 1.0, 1.0, 1.0),
		}
	}

	pub fn get_parent_id(&self) -> Option<usize> {
		self.parent_id
	}

	pub fn borrow_children_ids(&self) -> &Vec<usize> {
		&self.children_ids
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

	pub fn set_matrix(&mut self, matrix: &[f32; 16]) -> &mut Self {
		Matrix4::copy(&mut self.matrix, matrix);
		Matrix4::decompose(&mut self.position, &mut self.quaternion, &mut self.scale, &self.matrix);
		Euler::set_from_quaternion(&mut self.rotation, &self.quaternion);
		self
	}

	pub fn update_matrix(&mut self) -> &mut Self {
		Quaternion::set_from_euler(&mut self.quaternion, &self.rotation);
		Matrix4::compose(&mut self.matrix, &self.position, &self.quaternion, &self.scale);
		self
	}
}
