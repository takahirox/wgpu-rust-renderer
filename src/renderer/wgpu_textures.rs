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
	texture::texture::{
		Texture,
		TextureFormat,
	},
};

pub struct WGPUTextures {
	textures: HashMap<ResourceId<Texture>, wgpu::Texture>,
}

impl WGPUTextures {
	pub fn new() -> WGPUTextures {
		WGPUTextures {
			textures: HashMap::new(),
		}
	}

	pub fn borrow(&self, texture: &ResourceId<Texture>) -> Option<&wgpu::Texture> {
		self.textures.get(texture)
	}

	// @TODO: Implement correctly
	fn update(
		&mut self,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		pools: &ResourcePools,
		texture_rid: &ResourceId<Texture>,
	) {
		if !self.textures.contains_key(texture_rid) {
			if let Some(texture) = pools.borrow::<Texture>().borrow(texture_rid) {
				let texture_gpu = create_texture(
					device,
					texture.get_width(),
					texture.get_height(),
					get_wgpu_format(texture.borrow_format()),
				);
				upload_texture(
					queue,
					&texture_gpu,
					texture.get_width(),
					texture.get_height(),
					bytemuck::cast_slice(texture.borrow_texels()),
				);
				self.textures.insert(*texture_rid, texture_gpu);
			}
		}
	}

	pub fn update_from_material(
		&mut self,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		pools: &ResourcePools,
		material: &Material,
	) {
		let textures = material.borrow_textures(
			pools.borrow::<Box<dyn MaterialNode>>(),
		);
		for texture in textures.iter() {
			self.update(device, queue, pools, texture);
		}
	}
}

fn create_texture(
	device: &wgpu::Device,
	width: u32,
	height: u32,
	format: wgpu::TextureFormat
) -> wgpu::Texture {
	device.create_texture(&wgpu::TextureDescriptor {
		label: None,
		size: wgpu::Extent3d {
			width: width,
			height: height,
			depth_or_array_layers: 1,
        },
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: format,
		usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
	})
}

fn upload_texture(
	queue: &wgpu::Queue,
	texture: &wgpu::Texture,
	width: u32,
	height: u32,
	texels: &[u8],
) {
	queue.write_texture(
		texture.as_image_copy(),
		&texels,
		wgpu::ImageDataLayout {
			offset: 0,
			// @TODO: Fix me
			bytes_per_row: Some(std::num::NonZeroU32::new(width * 4).unwrap()),
			rows_per_image: None,
		},
		wgpu::Extent3d {
			width: width,
			height: height,
			depth_or_array_layers: 1,
        },
	);
}

fn get_wgpu_format(format: &TextureFormat) -> wgpu::TextureFormat {
	match format {
		TextureFormat::Float => wgpu::TextureFormat::Rgba32Float,
		TextureFormat::Uint8 => wgpu::TextureFormat::Rgba8Unorm,
		TextureFormat::Uint8Srgb => wgpu::TextureFormat::Rgba8UnormSrgb,
	}
}
