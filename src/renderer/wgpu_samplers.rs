use std::collections::HashMap;
use crate::{
	material::{
		material::Material,
		node::node::MaterialNode,
	},
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	texture::sampler::{
		FilterMode,
		Sampler,
		WrapMode,
	},
};

pub struct WGPUSamplers {
	samplers: HashMap<ResourceId<Sampler>, wgpu::Sampler>,
}

impl WGPUSamplers {
	pub fn new() -> Self {
		WGPUSamplers {
			samplers: HashMap::new(),
		}
	}

	pub fn borrow(&self, sampler: &ResourceId<Sampler>) -> Option<&wgpu::Sampler> {
		self.samplers.get(sampler)
	}

	// @TODO: Implement correctly
	fn update(
		&mut self,
		device: &wgpu::Device,
		pools: &ResourcePools,
		sampler_rid: &ResourceId<Sampler>,
	) {
		if !self.samplers.contains_key(sampler_rid) {
			if let Some(sampler) = pools.borrow::<Sampler>().borrow(sampler_rid) {
				let sampler_gpu = create_sampler(
					device,
					sampler,
				);
				self.samplers.insert(*sampler_rid, sampler_gpu);
			}
		}
	}

	pub fn update_from_material(
		&mut self,
		device: &wgpu::Device,
		pools: &ResourcePools,
		material: &Material,
	) {
		let samplers = material.borrow_samplers(
			pools.borrow::<Box<dyn MaterialNode>>(),
		);
		for sampler in samplers.iter() {
			self.update(device, pools, sampler);
		}
	}
}

fn create_sampler(
	device: &wgpu::Device,
	sampler: &Sampler,
) -> wgpu::Sampler {
	// @TODO: Fix me
	device.create_sampler(&wgpu::SamplerDescriptor {
		address_mode_u: get_address_mode(sampler.wrap_u()),
		address_mode_v: get_address_mode(sampler.wrap_v()),
		address_mode_w: get_address_mode(sampler.wrap_w()),
		anisotropy_clamp: None,
		border_color: None,
		compare: None,
		mag_filter: get_filter_mode(sampler.mag_filter()),
		min_filter: get_filter_mode(sampler.min_filter()),
		mipmap_filter: get_filter_mode(sampler.mipmap_filter()),
		label: None,
		lod_max_clamp: 0.0,
 		lod_min_clamp: 0.0,
	})
}

fn get_address_mode(mode: &WrapMode) -> wgpu::AddressMode {
	match mode {
		WrapMode::ClampToBorder => wgpu::AddressMode::ClampToBorder,
		WrapMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
		WrapMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
		WrapMode::Repeat => wgpu::AddressMode::Repeat,
	}
}

fn get_filter_mode(mode: &FilterMode) -> wgpu::FilterMode {
	match mode {
		FilterMode::Nearest => wgpu::FilterMode::Nearest,
		FilterMode::Linear => wgpu::FilterMode::Linear,
	}
}
