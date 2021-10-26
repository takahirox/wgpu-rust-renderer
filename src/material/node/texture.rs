use std::collections::HashMap;
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
}

impl TextureNode {
	pub fn new(
		texture: ResourceId<Texture>,
		sampler: ResourceId<Sampler>,
	) -> Self {
		TextureNode {
			contents: UniformContents::Texture {
				sampler: sampler,
				texture: texture,
			},
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
}

impl MaterialNode for TextureNode {
	fn collect_nodes (
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<ResourceId<Box<dyn MaterialNode>>>,
		visited: &mut HashMap<ResourceId<Box<dyn MaterialNode>>, bool>,
		self_rid: ResourceId<Box<dyn MaterialNode>>,
	) {
		if !visited.contains_key(&self_rid) {
			visited.insert(self_rid, true);
			nodes.push(self_rid);
		}
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		Some(&self.contents)
	}

	fn build_declaration(&self, _self_id: usize) -> String {
		format!("")
	}

	fn build_functions(&self, _self_id: usize) -> String {
		format!("")
	}

	fn build_fragment_shader(
		&self,
		_pool: &ResourcePool<Box<dyn MaterialNode>>,
		visited: &mut HashMap<usize, bool>,
		self_id: usize,
	) -> String {
		if visited.contains_key(&self_id) {
			return "".to_string();
		}
		visited.insert(self_id, true);

		format!("let {} = textureSample({}, {}, in.uv);\n",
			self.get_fragment_output(self_id),
			self.get_texture_name(),
			self.get_sampler_name(),
		)
	}

	fn get_fragment_output(&self, self_id: usize) -> String {
		format!("texture_output_{}", self_id)
	}
}
