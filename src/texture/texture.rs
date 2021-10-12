// @TODO: Support more format
pub enum TextureFormat {
	Float,
	Uint8,
}

// @TODO: Support 3D texture
pub struct Texture {
	format: TextureFormat,
	height: u32,
	id: usize,
	texels: Vec<u8>, // @TODO: Support shared texels?
	width: u32,
}

impl Texture {
	fn new(
		id: usize,
		width: u32,
		height: u32,
		format: TextureFormat,
		texels: Vec<u8>,
	) -> Self {
		Texture {
			format: format,
			height: height,
			id: id,
			texels: texels,
			width: width,
		}
	}

	pub fn get_id(&self) -> usize {
		self.id
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

// @TODO: Fix me.
pub struct TextureManager {
	count: usize
}

impl TextureManager {
	pub fn new() -> Self {
		TextureManager {
			count: 0
		}
	}

	pub fn create(
		&mut self,
		width: u32,	
		height: u32,
		format: TextureFormat,
		texels: Vec<u8>, // @TODO: Support shared texels?
	) -> Texture {
		let texture = Texture::new(
			self.count,
			width,
			height,
			format,
			texels,
		);
		self.count += 1;
		texture
	}

	// @TODO: Remove this temporal method.
	pub fn create_dummy(&mut self) -> Texture {
		self.create(
			256,
			256,
			TextureFormat::Uint8,
			vec![255; 256 * 256 * 4],
		)
	}
}
