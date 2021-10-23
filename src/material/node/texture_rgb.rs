use crate::{
	material::node::node::{
		MaterialNode,
		UniformContents,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
	texture::texture::Texture,
};

pub struct TextureRGBNode {
	contents: UniformContents,
	label: String,
}

impl TextureRGBNode {
	pub fn new(label: &str, texture: ResourceId<Texture>) -> Self {
		TextureRGBNode {
			contents: UniformContents::Texture {
				value: texture
			},
			label: label.to_string(),
		}
	}

	fn get_name(&self) -> String {
		format!("texture_{}", self.label)
	}
}

impl MaterialNode for TextureRGBNode {
	fn collect_nodes<'a> (
		&'a self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		_nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>) {
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		Some(&self.contents)
	}

	fn build_declaration(&self) -> String {
		format!("var {}: texture_2d<f32>;", self.get_name())
	}

	fn build_functions(&self) -> String {
		format!("")
	}

	fn build_fragment_shader(
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		format!("let {}_output = textureLoad({}, vec2<i32>(in.uv * 256.0), 0).rgb;\n",
			self.get_name(), self.get_name())
	}

	fn get_fragment_output(&self) -> String {
		format!("{}_output", self.get_name())
	}
}
