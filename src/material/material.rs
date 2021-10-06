use crate::{
	math::color::Color,
	scene::texture::Texture,
};

pub struct Material {
	color: [f32; 3],
	texture: Option<Texture>, // @TODO: Support shared texture
}

impl Material {
	pub fn new() -> Material {
		Material {
			color: Color::create(),
			texture: None,
		}
	}

	pub fn borrow_color(&self) -> &[f32; 3] {
		&self.color
	}

	pub fn borrow_color_mut(&mut self) -> &mut [f32; 3] {
		&mut self.color
	}

	pub fn borrow_texture(&self) -> Option<&Texture> {
		self.texture.as_ref()
	}

	pub fn set_texture(&mut self, texture: Option<Texture>) -> &mut Self {
		self.texture = texture;
		self
	}
}
