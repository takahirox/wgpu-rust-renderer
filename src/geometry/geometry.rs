use std::collections::HashMap;

use crate::{
	geometry::{
		attribute::Attribute,
		index::Index,
	},
	resource::resource::ResourceId,
};

// @TODO: Support shared attribute
pub struct Geometry {
	attributes: HashMap<&'static str, ResourceId<Attribute>>,
	index: Option<ResourceId<Index>>,
}

impl Geometry {
	pub fn new() -> Self {
		Geometry {
			attributes: HashMap::new(),
			index: None,
		}
	}

	pub fn set_attribute(&mut self, key: &'static str, attribute: ResourceId<Attribute>) -> &mut Self {
		self.attributes.insert(key, attribute);
		self
	}

	pub fn borrow_attribute(&self, key: &'static str) -> Option<&ResourceId<Attribute>> {
		self.attributes.get(key)
	}

	pub fn set_index(&mut self, index: ResourceId<Index>) -> &mut Self {
		self.index = Some(index);
		self
	}

	pub fn remove_index(&mut self) -> &mut Self {
		self.index = None;
		self
	}

	pub fn borrow_index(&self) -> Option<&ResourceId<Index>> {
		self.index.as_ref()
	}
}
