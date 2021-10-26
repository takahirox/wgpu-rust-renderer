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
			normal::NormalNode,
			texture::TextureNode,
			vector3::Vector3Node,
			xyz::XYZNode,
		},
	},
	math::color::Color,
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	texture::texture::Texture,
	utils::texture_loader::TextureLoader,
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
		let color = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(Vector3Node::new(
				*Color::copy(&mut Color::create(), color),
			),
		));

		let sampler = TextureLoader::create_default_sampler(pools);
		let texture = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(TextureNode::new(texture, sampler)),
		);

		let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(XYZNode::new(texture)),
		);

		let color_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(MultiplyNode::new(color, texture_rgb)),
		);

		pools.borrow_mut::<Material>().add(Material::new(color_node))
	}

	pub fn create_brdf_material(
		pools: &mut ResourcePools,
		color: &[f32; 3],
		metallic: f32,
		roughness: f32,
	) -> ResourceId<Material> {
		let base_color = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(Vector3Node::new(
				*Color::copy(&mut Color::create(), color),
			)),
		);

		let metallic = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(FloatNode::new(metallic)),
		);

		let roughness = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(FloatNode::new(roughness)),
		);

		let normal = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(NormalNode::new()),
		);

		let desc = BRDFNodeDescriptor {
			base_color: base_color,
			metallic: metallic,
			normal: normal,
			roughness: roughness,	
		};

		let brdf_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
			Box::new(BRDFNode::new(desc)),
		);

		pools.borrow_mut::<Material>().add(Material::new(brdf_node))
	}
}
