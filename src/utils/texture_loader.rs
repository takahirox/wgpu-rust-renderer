use crate::utils::file_loader::FileLoader;

use crate::{
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	texture::{
		sampler::{
			Sampler,
			SamplerDescriptor,
		},
		texture::{
			Texture,
			TextureFormat,
		},
	},
};

pub struct TextureLoader {
}

impl TextureLoader {
	pub fn load_png<R: std::io::Read>(
		pools: &mut ResourcePools,
		reader: R,
		// @TODO: Should use default rather than Option?
		format: Option<TextureFormat>,
	) -> ResourceId<Texture> {
		let decoder = png::Decoder::new(reader);
		let mut reader = decoder.read_info().unwrap();
		let (width, height) = {
			let info = reader.info();
			(info.width, info.height)
		};
		let mut buf = vec![0; reader.output_buffer_size()];
		reader.next_frame(&mut buf).unwrap();

		pools.borrow_mut::<Texture>().add(
			Texture::new(
				width,
				height,
				match format {
					Some(format) => format,
					None => TextureFormat::Uint8,
				},
				buf,
			)
		)
	}

	pub async fn load_png_with_filepath(
		pools: &mut ResourcePools,
		file_path: &str,
		format: Option<TextureFormat>,
	) -> ResourceId<Texture> {
		Self::load_png(pools, FileLoader::open(file_path).await, format)
	}

	pub fn load_jpg<R: std::io::Read>(
		pools: &mut ResourcePools,
		reader: R,
		format: Option<TextureFormat>,
	) -> ResourceId<Texture> {
		let mut decoder = jpeg_decoder::Decoder::new(reader);
		let pixels = decoder.decode().expect("failed to decode image");
		let (width, height) = {
			let metadata = decoder.info().unwrap();
			(metadata.width as u32, metadata.height as u32)
		};

		// @TODO: Fix me
		let mut data = Vec::new();
		for y in 0..height as usize {
			for x in 0..width as usize {
				data.push(pixels[(y * width as usize + x) * 3 + 0]);
				data.push(pixels[(y * width as usize + x) * 3 + 1]);
				data.push(pixels[(y * width as usize + x) * 3 + 2]);
				data.push(255);
			}
		}

		pools.borrow_mut::<Texture>().add(
			Texture::new(
				width,
				height,
				match format {
					Some(format) => format,
					None => TextureFormat::Uint8,
				},
				data,
			)
		)
	}

	pub async fn load_jpg_with_filepath(
		pools: &mut ResourcePools,
		file_path: &str,
		format: Option<TextureFormat>,
	) -> ResourceId<Texture> {
		Self::load_jpg(pools, FileLoader::open(file_path).await, format)
	}

	pub async fn load_with_filepath(
		pools: &mut ResourcePools,
		file_path: &str,
		format: Option<TextureFormat>,
	) -> ResourceId<Texture> {
		let path = std::path::Path::new(file_path);
		// @TODO: proper error handling
		match path.extension() {
			Some(extension) => match extension.to_str() {
				Some(str) => match str.to_lowercase().as_str() {
					"png" => Self::load_png_with_filepath(pools, file_path, format).await,
					"jpg" | "jpeg" => Self::load_jpg_with_filepath(pools, file_path, format).await,
					_ => panic!("Unknown texture image format, {:?}", extension),
				},
				None => panic!("Can not detect image file format from the file path, {}", file_path),
			},
			None => panic!("Can not detect image file format from the file path, {}", file_path),
		}
	}

	pub fn create_default_sampler(
		pools: &mut ResourcePools,
	) -> ResourceId<Sampler> {
		pools.borrow_mut::<Sampler>().add(
			Sampler::new(SamplerDescriptor {
				mag_filter: None,
				min_filter: None,
				mipmap_filter: None,
				wrap_u: None,
				wrap_v: None,
				wrap_w: None,
			})
		)
	}
}
