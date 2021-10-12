use std::collections::HashMap;

use crate::geometry::{
	attribute::Attribute,
	index::Index,
};

// @TODO: Support shared attribute
pub struct Geometry {
	attributes: HashMap<&'static str, Attribute>,
	index: Option<Index>,
}

impl Geometry {
	pub fn new() -> Self {
		Geometry {
			attributes: HashMap::new(),
			index: None,
		}
	}

	pub fn set_attribute(&mut self, key: &'static str, attribute: Attribute) -> &mut Self {
		self.attributes.insert(key, attribute);
		self
	}

	pub fn borrow_attribute(&self, key: &'static str) -> Option<&Attribute> {
		self.attributes.get(key)
	}

	pub fn set_index(&mut self, index: Index) -> &mut Self {
		self.index = Some(index);
		self
	}

	pub fn remove_index(&mut self) -> &mut Self {
		self.index = None;
		self
	}

	pub fn borrow_index(&self) -> Option<&Index> {
		self.index.as_ref()
	}
}
