use crate::{
	material::{
		material::Material,
		node::{
			MultiplyNode,
			TextureRGBNode,
			Vector3Node,
		},
	},
	math::color::Color,
	texture::texture::Texture,
};

pub struct MaterialHelper {
}

impl MaterialHelper {
	pub fn create_basic_material(
		color: &[f32; 3],
	) -> Material {
		let color_node = Box::new(Vector3Node::new(
			"color",
			*Color::copy(&mut Color::create(), color),
		));

		Material::new(color_node)
	}

	pub fn create_basic_material_with_texture(
		color: &[f32; 3],
		texture: Texture
	) -> Material {
		let color_node = Box::new(MultiplyNode::new(
			Box::new(Vector3Node::new(
				"color",
				*Color::copy(&mut Color::create(), color),
			)),
			Box::new(TextureRGBNode::new("color", texture)),
		));

		Material::new(color_node)
	}
}
