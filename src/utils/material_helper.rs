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
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	texture::texture::Texture,
};

pub struct MaterialHelper {
}

impl MaterialHelper {
	pub fn create_basic_material(
		pools: &mut ResourcePools,
		color: &[f32; 3],
	) -> ResourceId<Material> {
		let color_node = Box::new(Vector3Node::new(
			"color",
			*Color::copy(&mut Color::create(), color),
		));

		pools.borrow_mut::<Material>().add(Material::new(color_node))
	}

	pub fn create_basic_material_with_texture(
		pools: &mut ResourcePools,
		color: &[f32; 3],
		texture: ResourceId<Texture>,
	) -> ResourceId<Material> {
		let color_node = Box::new(MultiplyNode::new(
			Box::new(Vector3Node::new(
				"color",
				*Color::copy(&mut Color::create(), color),
			)),
			Box::new(TextureRGBNode::new("color", texture)),
		));

		pools.borrow_mut::<Material>().add(Material::new(color_node))
	}
}
