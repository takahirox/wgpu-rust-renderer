use std::collections::HashMap;

use crate::scene::attribute::Attribute;

pub struct WGPUAttributes {
	attributes: HashMap<usize, wgpu::Buffer> // attribute id -> wgpu buffer
}

impl WGPUAttributes {
	pub fn new() -> Self {
		WGPUAttributes {
			attributes: HashMap::new()
		}
	}

	pub fn borrow(&self, attribute: &Attribute) -> Option<&wgpu::Buffer> {
		self.attributes.get(&attribute.get_id())
	}

	// @TODO: Implement correctly
	pub fn update(&mut self, device: &wgpu::Device, attribute: &Attribute) {
		if !self.attributes.contains_key(&attribute.get_id()) {
			self.attributes.insert(attribute.get_id(), create_buffer(
				device,
				bytemuck::cast_slice(attribute.borrow_data()),
				wgpu::BufferUsages::VERTEX,
			));
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
