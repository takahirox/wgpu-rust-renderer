use crate::{
	material::{
		material::Material,
		node::{
			brdf::{
				BRDFNode,
				BRDFNodeDescriptor,
			},
			float::FloatNode,
			multiply::MultiplyNode,
			node::MaterialNode,
			texture_rgb::TextureRGBNode,
			vector3::Vector3Node,
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
		let color_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(Vector3Node::new(
				"color",
				*Color::copy(&mut Color::create(), color),
			)),
		);

		pools.borrow_mut::<Material>().add(Material::new(color_node))
	}

	pub fn create_basic_material_with_texture(
		pools: &mut ResourcePools,
		color: &[f32; 3],
		texture: ResourceId<Texture>,
	) -> ResourceId<Material> {
		let pool = pools.borrow_mut::<Box<dyn MaterialNode>>();

		let color = pool.add(Box::new(Vector3Node::new(
			"color",
			*Color::copy(&mut Color::create(), color),
		)));
		let texture = pool.add(Box::new(TextureRGBNode::new("color", texture)));

		let color_node = pool.add(Box::new(MultiplyNode::new(
			color,
			texture,
		)));

		pools.borrow_mut::<Material>().add(Material::new(color_node))
	}

	pub fn create_brdf_material(
		pools: &mut ResourcePools,
		color: &[f32; 3],
		metallic: f32,
		roughness: f32,
	) -> ResourceId<Material> {
		let pool = pools.borrow_mut::<Box<dyn MaterialNode>>();

		let base_color = pool.add(Box::new(Vector3Node::new(
			"color",
			*Color::copy(&mut Color::create(), color),
		)));
		let metallic = pool.add(Box::new(FloatNode::new("metallic", metallic)));
		let roughness = pool.add(Box::new(FloatNode::new("roughness", roughness)));

		let desc = BRDFNodeDescriptor {
			label: "brdf".to_string(),
			base_color: base_color,
			metallic: metallic,
			roughness: roughness,	
		};

		let brdf_node = pool.add(Box::new(BRDFNode::new(desc)));
		pools.borrow_mut::<Material>().add(Material::new(brdf_node))
	}
}
