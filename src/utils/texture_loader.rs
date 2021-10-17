use std::fs::File;

use crate::texture::texture::{
	Texture,
	TextureFormat,
};

pub struct TextureLoader {
}

impl TextureLoader {
	pub fn load_png(file_path: &str) -> Texture {
		let decoder = png::Decoder::new(File::open(file_path).unwrap());
		let mut reader = decoder.read_info().unwrap();
		let (width, height) = {
			let info = reader.info();
			(info.width, info.height)
		};
		let mut buf = vec![0; reader.output_buffer_size()];
		reader.next_frame(&mut buf).unwrap();

		Texture::new(
			width,
			height,
			TextureFormat::Uint8,
			buf,
		)
	}
}
