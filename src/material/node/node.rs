use crate::{
	resource::resource::{
		ResourceId,
		ResourcePool,
	},
	texture::texture::Texture,
};

pub enum UniformContents {
	Float {value: [f32; 1]},
	Matrix4 {value: [f32; 16]},
	Vector3 {value: [f32; 3]},
	Texture {value: ResourceId<Texture>},
}

pub trait MaterialNode {
	fn collect_nodes<'a> (
		&'a self,
		pool: &'a ResourcePool<Box<dyn MaterialNode>>,
		nodes: &mut Vec<&'a ResourceId<Box<dyn MaterialNode>>>,
	);
	fn borrow_contents(&self) -> Option<&UniformContents>;
	fn build_declaration(&self) -> String;
	fn build_functions(&self) -> String;
	fn build_fragment_shader(
		&self,
		pool: &ResourcePool<Box<dyn MaterialNode>>,
	) -> String;
	fn get_fragment_output(&self) -> String;
}

// @TODO: Ensure unique variable names
