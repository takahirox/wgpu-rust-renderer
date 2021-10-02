pub struct Object {
	children_ids: Vec<usize>,
	id: usize,
	matrix: [f32; 16],
	parent_id: Option<usize>, 
	position: [f32; 3],
	_quaternion: [f32; 4],
	rotation: [f32; 3],
	scale: [f32; 3],
}

impl Object {
	pub fn new(id: usize) -> Self {
		Object {
			children_ids: Vec::new(),
			id: id,
			matrix: [
				1.0, 0.0, 0.0, 0.0,
				0.0, 1.0, 0.0, 0.0,
				0.0, 0.0, 1.0, 0.0,
				0.0, 0.0, 0.0, 1.0,
			],
			parent_id: None,
			position: [0.0, 0.0, 0.0],
			_quaternion: [0.0, 0.0, 0.0, 1.0],
			rotation: [0.0, 0.0, 0.0],
			scale: [1.0, 1.0, 1.0],
		}
	}

	pub fn get_id(&self) -> usize {
		self.id
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

	pub fn borrow_rotation(&self) -> &[f32; 3] {
		&self.rotation
	}

	pub fn borrow_scale(&self) -> &[f32; 3] {
		&self.scale
	}

	pub fn borrow_matrix(&self) -> &[f32; 16] {
		&self.matrix
	}
}
