use gltf::Gltf;

use crate::{
	geometry::{
		attribute::Attribute,
		geometry::Geometry,
		index::Index,
	},
	material::{
		material::Material,
		node::{
			add::AddNode,
			brdf::{
				BRDFNode,
				BRDFNodeDescriptor,
			},
			const_float::ConstFloatNode,
			float::FloatNode,
			multiply::MultiplyNode,
			node::MaterialNode,
			normal::NormalNode,
			srgb_to_linear::SRGBToLinearNode,
			sub::SubNode,
			tangent_to_object_normal::TangentToObjectNormalNode,
			texture::TextureNode,
			vector3::Vector3Node,
			xyz::XYZNode,
			y::YNode,
			z::ZNode,
		},
	},
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	scene::{
		mesh::Mesh,
		node::Node,
		scene::Scene,
	},
	texture::{
		sampler::{
			FilterMode,
			Sampler,
			SamplerDescriptor,
			WrapMode
		},
		texture::Texture,
	},
	utils::texture_loader::TextureLoader,
};

fn parse_attribute(
	pools: &mut ResourcePools,
	path: &str,
	primitive: &gltf::Attribute,
) -> (&'static str, ResourceId<Attribute>) {
	let (semantic, accessor) = primitive;
	use gltf::mesh::Semantic;
	if let Some(view) = accessor.view() {
		let offset = view.offset();
		let length = view.length();
		let buffer = view.buffer();

		use gltf::buffer::Source;
		use std::io::{Read, Seek, SeekFrom};
		let data = match buffer.source() {
			Source::Bin => {
				panic!("Bin is not supported yet");
			},
			Source::Uri(uri) => {
				let mut buf = [0_u8; 4];
				let mut data = Vec::<f32>::new();
				let mut file = std::fs::File::open(
					path.to_owned() + uri,
				).unwrap();
				for i in 0..(length / 4) {
					file.seek(SeekFrom::Start((offset + i * 4) as u64)).unwrap();
					file.read_exact(&mut buf).unwrap();
					data.push(f32::from_le_bytes(buf));
				}
				data
			}
		};

		let (name, attribute) = match semantic {
			Semantic::Normals => {(
				"normal",
				Attribute::new(data, 3),
			)},
			Semantic::Positions => {(
				"position",
				Attribute::new(data, 3),
			)},
			Semantic::TexCoords(_) => {(
				"uv",
				Attribute::new(data, 2),
			)},
			_ => {
				panic!("Unsupport accessor semantic.");
			},
		};

		(name, pools.borrow_mut::<Attribute>().add(attribute))
	} else {
		panic!("Sparse accessor is not supported yet.");
	}
}

fn parse_normal_texture_info(
	pools: &mut ResourcePools,
	path: &str,
	info: &gltf::material::NormalTexture,
) -> (ResourceId<Texture>, ResourceId<Sampler>) {
	parse_texture(pools, path, &info.texture())
}

fn parse_index(
	pools: &mut ResourcePools,
	path: &str,
	index: &gltf::Accessor,
) -> ResourceId<Index> {
	if let Some(view) = index.view() {
		let offset = view.offset();
		let length = view.length();
		let buffer = view.buffer();

		use gltf::buffer::Source;
		use std::io::{Read, Seek, SeekFrom};
		let data = match buffer.source() {
			Source::Bin => {
				panic!("Bin is not supported yet");
			},
			Source::Uri(uri) => {
				let mut buf = [0_u8; 2];
				let mut data = Vec::<u16>::new();
				let mut file = std::fs::File::open(
					path.to_owned() + uri,
				).unwrap();
				for i in 0..(length / 2) {
					file.seek(SeekFrom::Start((offset + i * 2) as u64)).unwrap();
					file.read_exact(&mut buf).unwrap();
					data.push(u16::from_le_bytes(buf));
				}
				data
			}
		};

		pools.borrow_mut::<Index>().add(Index::new(data))
	} else {
		panic!("Sparse accessor is not supported yet.");
	}
}

fn parse_sampler(
	pools: &mut ResourcePools,
	sampler: &gltf::texture::Sampler,
) -> ResourceId<Sampler> {
	// @TODO: Proper default values
	pools.borrow_mut::<Sampler>().add(Sampler::new(
		SamplerDescriptor {
			mag_filter: match sampler.mag_filter() {
				Some(filter) => match filter {
					gltf::texture::MagFilter::Nearest => Some(FilterMode::Nearest),
					gltf::texture::MagFilter::Linear => Some(FilterMode::Linear),
				},
				None => None,
			},
			min_filter: match sampler.min_filter() {
				Some(filter) => match filter {
					gltf::texture::MinFilter::Linear |
					gltf::texture::MinFilter::LinearMipmapLinear |
					gltf::texture::MinFilter::LinearMipmapNearest => Some(FilterMode::Linear),
					gltf::texture::MinFilter::Nearest |
					gltf::texture::MinFilter::NearestMipmapLinear |
					gltf::texture::MinFilter::NearestMipmapNearest => Some(FilterMode::Nearest),
				},
				None => None,
			},
			mipmap_filter: match sampler.min_filter() {
				Some(filter) => match filter {
					gltf::texture::MinFilter::Linear |
					gltf::texture::MinFilter::Nearest => None,
					gltf::texture::MinFilter::LinearMipmapLinear |
					gltf::texture::MinFilter::NearestMipmapLinear => Some(FilterMode::Linear),
					gltf::texture::MinFilter::LinearMipmapNearest |
					gltf::texture::MinFilter::NearestMipmapNearest => Some(FilterMode::Nearest),
				},
				None => None,
			},
			wrap_u: match sampler.wrap_s() {
				gltf::texture::WrappingMode::ClampToEdge => Some(WrapMode::ClampToEdge),
				gltf::texture::WrappingMode::MirroredRepeat => Some(WrapMode::MirrorRepeat),
				gltf::texture::WrappingMode::Repeat => Some(WrapMode::Repeat),
			},
			wrap_v: match sampler.wrap_t() {
				gltf::texture::WrappingMode::ClampToEdge => Some(WrapMode::ClampToEdge),
				gltf::texture::WrappingMode::MirroredRepeat => Some(WrapMode::MirrorRepeat),
				gltf::texture::WrappingMode::Repeat => Some(WrapMode::Repeat),
			},
			wrap_w: None,
		},
	))
}

fn parse_texture(
	pools: &mut ResourcePools,
	path: &str,
	texture_def: &gltf::Texture,
) -> (ResourceId<Texture>, ResourceId<Sampler>) {
	let source = texture_def.source();

	use gltf::image::Source;
	let texture = match source.source() {
		Source::Uri {uri, mime_type: _mime_type} => {
			// @Support PNG
			TextureLoader::load_jpg_with_filepath(
				pools,
				&(path.to_owned() + uri),
			)
		},
		Source::View {..} => {
			panic!("Unsuppored");
		},
	};

	(texture, parse_sampler(pools, &texture_def.sampler()))
}

fn parse_texture_info(
	pools: &mut ResourcePools,
	path: &str,
	info: &gltf::texture::Info,
) -> (ResourceId<Texture>, ResourceId<Sampler>) {
	parse_texture(pools, path, &info.texture())
}

pub struct GltfLoader{
}

// @TODO: Clean up
// @TODO: Wasm support
impl GltfLoader {
	pub fn load_gltf(
		pools: &mut ResourcePools,
		scene: &ResourceId<Scene>,
		path: &str,
		filename: &str,
	) -> Vec<ResourceId<Node>> {
		let gltf = Gltf::open(path.to_owned() + filename).unwrap();
		let mut nodes = Vec::new();

		let scene_def = gltf.default_scene().unwrap();
		for node_def in scene_def.nodes() {
			let node = pools.borrow_mut::<Node>().add(Node::new());
			if let Some(mesh) = node_def.mesh() {
				for primitive in mesh.primitives() {
					let mut geometry = Geometry::new();

					for attribute_def in primitive.attributes() {
						let (name, attribute) = parse_attribute(pools, path, &attribute_def);
						geometry.set_attribute(&name, attribute);
					}

					if let Some(accessor) = primitive.indices() {
						let index = parse_index(pools, path, &accessor);
						geometry.set_index(index);
					}

					let geometry = pools.borrow_mut::<Geometry>().add(geometry);

					let material_def = primitive.material();
					let pbr_metallic_roughness = material_def.pbr_metallic_roughness();

					let base_color_factor = pbr_metallic_roughness.base_color_factor();
					let metallic_factor = pbr_metallic_roughness.metallic_factor();
					let roughness_factor = pbr_metallic_roughness.roughness_factor();

					let base_color = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						Vector3Node::new(
							[base_color_factor[0], base_color_factor[1], base_color_factor[2]],
						),
					));

					let metallic = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
						Box::new(FloatNode::new(metallic_factor)),
					);

					let roughness = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
						Box::new(FloatNode::new(roughness_factor)),
					);

					let base_color = if let Some(info) = pbr_metallic_roughness.base_color_texture() {
						let (texture, sampler) = parse_texture_info(pools, path, &info);

						let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(TextureNode::new(texture, sampler)),
						);

						let linear_texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(SRGBToLinearNode::new(texture_node)),
						);

						let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(XYZNode::new(linear_texture_node)),
						);

						pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(MultiplyNode::new(base_color, texture_rgb))
						)
					} else {
						base_color
					};

					let (metallic, roughness) = if let Some(info) = pbr_metallic_roughness.metallic_roughness_texture() {
						let (texture, sampler) = parse_texture_info(pools, path, &info);

						let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(TextureNode::new(texture, sampler)),
						);

						let texture_g = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(YNode::new(texture_node)),
						);

						let texture_b = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(ZNode::new(texture_node)),
						);

						let metallic = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(MultiplyNode::new(metallic, texture_b)),
						);

						let roughness = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(MultiplyNode::new(roughness, texture_g)),
						);

						(metallic, roughness)
					} else {
						(metallic, roughness)
					};

					let normal = if let Some(info) = material_def.normal_texture() {
						let (texture, sampler) = parse_normal_texture_info(pools, path, &info);

						let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(TextureNode::new(texture, sampler)),
						);

						let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(XYZNode::new(texture_node)),
						);

						let const_2 = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(ConstFloatNode::new(2.0)),
						);

						let const_1 = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(ConstFloatNode::new(1.0)),
						);

						let multiply = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(MultiplyNode::new(texture_rgb, const_2)),
						);

						let sub = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(SubNode::new(multiply, const_1)),
						);

						pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(TangentToObjectNormalNode::new(sub)),
						)
					} else {
						pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
							NormalNode::new()
						))
					};

					let brdf = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						BRDFNode::new(BRDFNodeDescriptor {
							base_color: base_color,
							metallic: metallic,
							normal: normal,
							roughness: roughness,
						}),
					));

					let emissive_factor = material_def.emissive_factor();
					let emissive = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						Vector3Node::new(
							[emissive_factor[0], emissive_factor[1], emissive_factor[2]],
						),
					));

					let emissive = if let Some(info) = material_def.emissive_texture() {
						let (texture, sampler) = parse_texture_info(pools, path, &info);

						let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(TextureNode::new(texture, sampler)),
						);

						let linear_texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(SRGBToLinearNode::new(texture_node)),
						);

						let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(XYZNode::new(linear_texture_node)),
						);

						pools.borrow_mut::<Box<dyn MaterialNode>>().add(
							Box::new(MultiplyNode::new(emissive, texture_rgb))
						)
					} else {
						emissive
					};

					let add = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						AddNode::new(
							brdf,
							emissive,
						),
					));

					let material = pools.borrow_mut::<Material>().add(Material::new(add));
					let mesh = pools.borrow_mut::<Mesh>().add(Mesh::new(geometry, material));

					pools.borrow_mut::<Scene>().borrow_mut(scene).unwrap().assign(&node, &mesh);
				}
			}
			nodes.push(node);
		}
		nodes
	}
}