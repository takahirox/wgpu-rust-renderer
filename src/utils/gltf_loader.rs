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
			brdf::{
				BRDFNode,
				BRDFNodeDescriptor,
			},
			float::FloatNode,
			multiply::MultiplyNode,
			node::MaterialNode,
			texture::TextureNode,
			vector3::Vector3Node,
			xyz::XYZNode,
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
	},
	utils::texture_loader::TextureLoader,
};

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

					for (semantic, accessor) in primitive.attributes() {
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
								Semantic::Normals => {
									(
										"normal",
										Attribute::new(data, 3),
									)
								},
								Semantic::Positions => {
									(
										"position",
										Attribute::new(data, 3),
									)
								},
								Semantic::TexCoords(_) => {
									(
										"uv",
										Attribute::new(data, 2),
									)
								},
								_ => {
									panic!("Unsupport accessor semantic");
								},
							};

							geometry.set_attribute(
								name,
								pools.borrow_mut::<Attribute>().add(attribute),
							);
						}
					}

					if let Some(indices) = primitive.indices() {
						if let Some(view) = indices.view() {
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

							let index = Index::new(data);
							geometry.set_index(
								pools.borrow_mut::<Index>().add(index),
							);
						}
					}

					let geometry = pools.borrow_mut::<Geometry>().add(geometry);

					let material_def = primitive.material();
					let pbr_metallic_roughness = material_def.pbr_metallic_roughness();
					let base_color_factor = pbr_metallic_roughness.base_color_factor();
					let metallic_factor = pbr_metallic_roughness.metallic_factor();
					let roughness_factor = pbr_metallic_roughness.roughness_factor();

					let base_color = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						Vector3Node::new(
							"base_color",
							[base_color_factor[0], base_color_factor[1], base_color_factor[2]],
						),
					));

					let metallic = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						FloatNode::new(
							"metallic",
							metallic_factor,
						),
					));

					let roughness = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						FloatNode::new(
							"roughness",
							roughness_factor,
						),
					));

					let base_color = if let Some(info) = pbr_metallic_roughness.base_color_texture() {
						let texture_def = info.texture();
						let sampler = texture_def.sampler();
						let source = texture_def.source();

						use gltf::image::Source;
						let texture = match source.source() {
							Source::View {..} => {
								panic!("Unsuppored");
							},
							Source::Uri {uri, mime_type: _mime_type} => {
								// @Support PNG
								TextureLoader::load_jpg_with_filepath(
									pools,
									&(path.to_owned() + uri),
								)
							},
						};

						let sampler = pools.borrow_mut::<Sampler>().add(Sampler::new(
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
						));

						let texture_node = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
							TextureNode::new(
								"base_color_texture",
								texture,
								sampler,
							)
						));

						let texture_rgb = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
							XYZNode::new(
								texture_node,
							)
						));

						pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
							MultiplyNode::new(
								base_color,
								texture_rgb,
							)
						))
					} else {
						base_color
					};

					let brdf = pools.borrow_mut::<Box<dyn MaterialNode>>().add(Box::new(
						BRDFNode::new(BRDFNodeDescriptor {
							label: "brdf".to_string(),
							base_color: base_color,
							metallic: metallic,
							roughness: roughness,
						}),
					));

					let material = pools.borrow_mut::<Material>().add(Material::new(brdf));
					let mesh = pools.borrow_mut::<Mesh>().add(Mesh::new(geometry, material));

					pools.borrow_mut::<Scene>().borrow_mut(scene).unwrap().assign(&node, &mesh);
				}
			}
			nodes.push(node);
		}
		nodes
	}
}