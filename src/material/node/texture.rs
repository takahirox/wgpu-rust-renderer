use crate::{
	material::node::node::{
		MaterialNode,
		UniformContents,
	},
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
	texture::{
		sampler::Sampler,
		texture::Texture,
	}
};

pub struct TextureNode {
	contents: UniformContents,
	_label: String,
}

impl TextureNode {
	pub fn new(
		label: &str,
		texture: ResourceId<Texture>,
		sampler: ResourceId<Sampler>,
	) -> Self {
		TextureNode {
			contents: UniformContents::Texture {
				sampler: sampler,
				texture: texture,
			},
			_label: label.to_string(),
		}
	}

	fn get_texture_name(&self) -> String {
		match self.contents {
			UniformContents::Texture{texture, ..} => {
				format!("texture_{}", texture.id)
			},
			_ => panic!(),		
		}
	}

	fn get_sampler_name(&self) -> String {
		match self.contents {
			UniformContents::Texture{sampler, ..} => {
				format!("sampler_{}", sampler.id)
			},
			_ => panic!(),		
		}
	}

	fn get_prefix(&self) -> String {
		match self.contents {
			UniformContents::Texture{sampler, texture} => {
				format!("texture_{}_{}", texture.id, sampler.id)
			},
			_ => panic!(),		
		}
	}
}

impl MaterialNode for TextureNode {
	fn collect_nodes<'a> (
		&'a self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		_nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>) {
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		Some(&self.contents)
	}

	fn build_declaration(&self) -> String {
		format!("")
	}

	fn build_functions(&self) -> String {
		format!("")
	}

	fn build_fragment_shader(
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String {
		format!("let {}_output = textureSample({}, {}, in.uv);\n",
			self.get_prefix(),
			self.get_texture_name(),
			self.get_sampler_name())
	}

	fn get_fragment_output(&self) -> String {
		format!("{}_output", self.get_prefix())
	}
}
