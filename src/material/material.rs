use crate::math::color::Color;

pub struct Material {
	color: [f32; 3]
}

impl Material {
	pub fn new() -> Material {
		Material {
			color: Color::create()
		}
	}

	pub fn borrow_color(&self) -> &[f32; 3] {
		&self.color
	}

	pub fn borrow_color_mut(&mut self) -> &mut [f32; 3] {
		&mut self.color
	}
}
