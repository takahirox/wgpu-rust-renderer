// @TODO: Support more format
pub enum TextureFormat {
	Float,
	Uint8,
	Uint8Srgb,
}

// @TODO: Support 3D texture
pub struct Texture {
	format: TextureFormat,
	height: u32,
	texels: Vec<u8>, // @TODO: Support shared texels?
	width: u32,
}

impl Texture {
	pub fn new(
		width: u32,
		height: u32,
		format: TextureFormat,
		texels: Vec<u8>,
	) -> Self {
		Texture {
			format: format,
			height: height,
			texels: texels,
			width: width,
		}
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}

	pub fn borrow_format(&self) -> &TextureFormat {
		&self.format
	}

	pub fn borrow_texels(&self) -> &Vec<u8> {
		&self.texels
	}
}
