use std::collections::HashMap;
use crate::{
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
	texture::{
		sampler::Sampler,
		texture::Texture,
	},
};

pub enum UniformContents {
	Float {value: [f32; 1]},
	Matrix4 {value: [f32; 16]},
	Vector3 {value: [f32; 3]},
	Texture {
		texture: ResourceId<Texture>,
		sampler: ResourceId<Sampler>,
	},
}

pub trait MaterialNode {
	fn collect_nodes (
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<ResourceId<Box<dyn MaterialNode>>>,
		visited: &mut HashMap<ResourceId<Box<dyn MaterialNode>>, bool>,
		self_rid: ResourceId<Box<dyn MaterialNode>>,
	);
	fn borrow_contents(&self) -> Option<&UniformContents>;
	fn build_declaration(&self, self_id: usize) -> String;
	fn build_functions(&self, self_id: usize) -> String;
	fn build_fragment_shader(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
		visited: &mut HashMap<usize, bool>,
		self_id: usize,
	) -> String;
	fn get_fragment_output(&self, self_id: usize) -> String;
}

// @TODO: Ensure unique variable names
