use std::fs::File;

use crate::scene::texture::{
	Texture,
	TextureFormat,
	TextureManager,
};

pub struct TextureLoader {
}

impl TextureLoader {
	pub fn load_png(manager: &mut TextureManager, file_path: &str) -> Texture {
		let decoder = png::Decoder::new(File::open(file_path).unwrap());
		let mut reader = decoder.read_info().unwrap();
		let (width, height) = {
			let info = reader.info();
			(info.width, info.height)
		};
		let mut buf = vec![0; reader.output_buffer_size()];
		reader.next_frame(&mut buf).unwrap();

		manager.create(
			width,
			height,
			TextureFormat::Uint8,
			buf,
		)
	}
}
