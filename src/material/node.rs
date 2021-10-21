use crate::{
	resource::resource::ResourceId,
	texture::texture::Texture,
};

pub enum UniformContents {
	Matrix4 {value: [f32; 16]},
	Vector3 {value: [f32; 3]},
	Texture {value: ResourceId<Texture>},
}

pub trait MaterialNode {
	fn collect_leaf_nodes<'a> (&'a self, nodes: &mut Vec<&'a dyn MaterialNode>);
	fn borrow_contents(&self) -> Option<&UniformContents>;
	fn build_declaration(&self) -> String;
	fn build_fragment_shader(&self) -> String;
	fn get_fragment_output(&self) -> String;
}

// @TODO: Ensure unique variable names

pub struct Vector3Node {
	contents: UniformContents,
	label: String,
}

impl Vector3Node {
	pub fn new(label: &str, value: [f32; 3]) -> Self {
		Vector3Node {
			contents: UniformContents::Vector3 {
				value: value,
			},
			label: label.to_string(),
		}
	}

	fn get_name(&self) -> String {
		format!("vec3_{}", &self.label)
	}
}

impl MaterialNode for Vector3Node {
	fn collect_leaf_nodes<'a> (&'a self, nodes: &mut Vec<&'a dyn MaterialNode>) {
		nodes.push(self as &dyn MaterialNode);
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		Some(&self.contents)
	}

	fn build_declaration(&self) -> String {
		format!("{}: vec3<f32>;\n", self.get_name())
	}

	fn build_fragment_shader(&self) -> String {
		format!("let {}_output = unif.{};\n", self.get_name(), self.get_name())
	}

	fn get_fragment_output(&self) -> String {
		format!("{}_output", self.get_name())
	}
}

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
	fn collect_leaf_nodes<'a> (&'a self, nodes: &mut Vec<&'a dyn MaterialNode>) {
		nodes.push(self as &dyn MaterialNode);
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		Some(&self.contents)
	}

	fn build_declaration(&self) -> String {
		format!("var {}: texture_2d<f32>;", self.get_name())
	}

	fn build_fragment_shader(&self) -> String {
		format!("let {}_output = textureLoad({}, vec2<i32>(in.uv * 256.0), 0).rgb;\n",
			self.get_name(), self.get_name())
	}

	fn get_fragment_output(&self) -> String {
		format!("{}_output", self.get_name())
	}
}

pub struct MultiplyNode {
	value1: Box<dyn MaterialNode>,
	value2: Box<dyn MaterialNode>,
}

impl MultiplyNode {
	pub fn new(value1: Box<dyn MaterialNode>, value2: Box<dyn MaterialNode>) -> Self {
		MultiplyNode {
			value1: value1,
			value2: value2,
		}
	}
}

impl MaterialNode for MultiplyNode {
	fn collect_leaf_nodes<'a> (&'a self, nodes: &mut Vec<&'a dyn MaterialNode>) {
		self.value1.collect_leaf_nodes(nodes);
		self.value2.collect_leaf_nodes(nodes);
	}

	fn borrow_contents(&self) -> Option<&UniformContents> {
		None
	}

	fn build_declaration(&self) -> String {
		format!("")
	}

	fn build_fragment_shader(&self) -> String {
		self.value1.build_fragment_shader() +
		&self.value2.build_fragment_shader() +
		&format!("let multiply_output = {} * {};\n",
			self.value1.get_fragment_output(),
			self.value2.get_fragment_output())
	}

	fn get_fragment_output(&self) -> String {
		format!("multiply_output")
	}
}
