use std::collections::HashMap;
use crate::scene::texture::Texture;

// @TODO: Fix me. We have a bad assumption that texture is always RGBA float 256x256
const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

pub struct WGPUTextures {
	textures: HashMap<usize, wgpu::Texture>,
}

impl WGPUTextures {
	pub fn new() -> WGPUTextures {
		WGPUTextures {
			textures: HashMap::new(),
		}
	}

	pub fn borrow(&self, texture: &Texture) -> Option<&wgpu::Texture> {
		self.textures.get(&texture.get_id())
	}

	// @TODO: Implement correctly
	pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, texture: &Texture) {
		if !self.textures.contains_key(&texture.get_id()) {
			let texture_gpu = create_texture(
				device,
				WIDTH,
				HEIGHT,
				FORMAT,
			);
			upload_texture(
				queue,
				&texture_gpu,
				WIDTH,
				HEIGHT,
				bytemuck::cast_slice(texture.borrow_texels()),
			);
			self.textures.insert(texture.get_id(), texture_gpu);
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
