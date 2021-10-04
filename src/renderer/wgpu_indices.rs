// @TODO: Should we reuse wgpu_attributes?

use std::collections::HashMap;

use crate::scene::index::Index;

pub struct WGPUIndices {
	indices: HashMap<usize, wgpu::Buffer> // index attribute id -> wgpu buffer
}

impl WGPUIndices {
	pub fn new() -> Self {
		WGPUIndices {
			indices: HashMap::new()
		}
	}

	pub fn borrow(&self, index: &Index) -> Option<&wgpu::Buffer> {
		self.indices.get(&index.get_id())
	}

	// @TODO: Implement correctly
	pub fn update(&mut self, device: &wgpu::Device, index: &Index) {
		if !self.indices.contains_key(&index.get_id()) {
			self.indices.insert(index.get_id(), create_buffer(
				device,
				bytemuck::cast_slice(index.borrow_data()),
				wgpu::BufferUsages::INDEX,
			));
		}
	}
}

// @TODO: Remove duplication with wgpu_attributes.rs
fn create_buffer(device: &wgpu::Device, contents: &[u8], usage: wgpu::BufferUsages) -> wgpu::Buffer {
	use wgpu::util::DeviceExt;
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: None,
		contents:  contents,
		usage: usage,
	})
}
