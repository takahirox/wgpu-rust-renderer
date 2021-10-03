use crate::math::matrix4::Matrix4;

pub struct PerspectiveCamera {
	aspect: f32,
	far: f32,
	fovy: f32,
	near: f32,
	projection_matrix: [f32; 16],
	projection_matrix_inverse: [f32; 16],
}

impl PerspectiveCamera {
	pub fn new(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
		let mut camera = PerspectiveCamera {
			aspect: aspect,
			far: far,
			fovy: fovy,
			near: near,
			projection_matrix: Matrix4::create(),
			projection_matrix_inverse: Matrix4::create(),
		};
		camera.update_projection_matrix();
		camera
	}

	pub fn set_aspect(&mut self, aspect: f32) -> &mut Self {
		self.aspect = aspect;
		self.update_projection_matrix();
		self
	}

	pub fn update_projection_matrix(&mut self) {
		Matrix4::make_perspective(
			&mut self.projection_matrix,
			self.fovy,
			self.aspect,
			self.near,
			self.far,
		);
		Matrix4::invert(
			Matrix4::copy(&mut self.projection_matrix_inverse, &self.projection_matrix)
		);
	}

	pub fn borrow_projection_matrix(&self) -> &[f32; 16] {
		&self.projection_matrix
	}

	pub fn borrow_projection_matrix_inverse(&self) -> &[f32; 16] {
		&self.projection_matrix_inverse
	}
}