pub struct Attribute {
	count: u32,
	data: Vec<f32>,
	_item_size: u32,
}

impl Attribute {
	pub fn new(data: Vec<f32>, item_size: u32) -> Self {
		Attribute {
			count: data.len() as u32 / item_size,
			data: data,
			_item_size: item_size,
		}
	}

	pub fn get_count(&self) -> u32 {
		self.count
	}

	pub fn borrow_data(&self) -> &Vec<f32> {
		&self.data
	}
}
