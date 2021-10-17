use std::collections::HashMap;

use crate::{
	geometry::attribute::Attribute,
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
};

pub struct WGPUAttributes {
	attributes: HashMap<ResourceId<Attribute>, wgpu::Buffer>,
}

impl WGPUAttributes {
	pub fn new() -> Self {
		WGPUAttributes {
			attributes: HashMap::new()
		}
	}

	pub fn borrow(&self, attribute: &ResourceId<Attribute>) -> Option<&wgpu::Buffer> {
		self.attributes.get(attribute)
	}

	// @TODO: Implement correctly
	pub fn update(
		&mut self,
		device: &wgpu::Device,
		pools: &ResourcePools,
		attribute_rid: &ResourceId<Attribute>,
	) {
		if !self.attributes.contains_key(attribute_rid) {
			if let Some(attribute) = pools.borrow::<Attribute>().borrow(attribute_rid) {
				self.attributes.insert(*attribute_rid, create_buffer(
					device,
					bytemuck::cast_slice(attribute.borrow_data()),
					wgpu::BufferUsages::VERTEX,
				));
			}
		}
	}
}

fn create_buffer(device: &wgpu::Device, contents: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
	use wgpu::util::DeviceExt;
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: None,
		contents:  contents,
		usage: usage,
	})
}
