use std::collections::HashMap;

use wgpu::util::DeviceExt;

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
			self.attributes.insert(attribute.get_id(), create_buffer(device, attribute.borrow_data()));
		}
	}
}

fn create_buffer(device: &wgpu::Device, data: &[f32]) -> wgpu::Buffer {
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: None,
		contents: bytemuck::cast_slice(data),
		usage: wgpu::BufferUsages::VERTEX,
	})
}
