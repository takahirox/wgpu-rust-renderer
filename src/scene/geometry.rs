use std::collections::HashMap;

use crate::scene::attribute::Attribute;

// @TODO: Support shared attribute
pub struct Geometry {
	attributes: HashMap<&'static str, Attribute>
}

impl Geometry {
	pub fn new() -> Self {
		Geometry {
			attributes: HashMap::new()
		}
	}

	pub fn set_attribute(&mut self, key: &'static str, attribute: Attribute) -> &mut Self {
		self.attributes.insert(key, attribute);
		self
	}

	pub fn borrow_attribute(&self, key: &'static str) -> Option<&Attribute> {
		self.attributes.get(key)
	}
}
