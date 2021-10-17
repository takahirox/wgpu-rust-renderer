// @TODO: Should we reuse wgpu_attributes?

use std::collections::HashMap;

use crate::{
	geometry::index::Index,
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
};

pub struct WGPUIndices {
	indices: HashMap<ResourceId<Index>, wgpu::Buffer>,
}

impl WGPUIndices {
	pub fn new() -> Self {
		WGPUIndices {
			indices: HashMap::new()
		}
	}

	pub fn borrow(&self, index: &ResourceId<Index>) -> Option<&wgpu::Buffer> {
		self.indices.get(index)
	}

	// @TODO: Implement correctly
	pub fn update(
		&mut self,
		device: &wgpu::Device,
		pools: &ResourcePools,
		index_rid: &ResourceId<Index>,
	) {
		if !self.indices.contains_key(index_rid) {
			if let Some(index) = pools.borrow::<Index>().borrow(index_rid) {
				self.indices.insert(*index_rid, create_buffer(
					device,
					bytemuck::cast_slice(index.borrow_data()),
					wgpu::BufferUsages::INDEX,
				));
			}
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
