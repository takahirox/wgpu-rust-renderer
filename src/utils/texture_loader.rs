use std::fs::File;

use crate::{
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	texture::texture::{
		Texture,
		TextureFormat,
	},
};

pub struct TextureLoader {
}

impl TextureLoader {
	pub fn load_png(pools: &mut ResourcePools, file_path: &str) -> ResourceId<Texture> {
		let decoder = png::Decoder::new(File::open(file_path).unwrap());
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
				TextureFormat::Uint8,
				buf,
			)
		)
	}
}
