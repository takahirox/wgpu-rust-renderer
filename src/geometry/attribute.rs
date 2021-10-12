pub struct Attribute {
	count: u32,
	data: Vec<f32>,
	id: usize,
	_item_size: u32,
}

impl Attribute {
	fn new(id: usize, data: Vec<f32>, item_size: u32) -> Self {
		Attribute {
			count: data.len() as u32 / item_size,
			data: data,
			id: id,
			_item_size: item_size,
		}
	}

	pub fn get_id(&self) -> usize {
		self.id
	}

	pub fn get_count(&self) -> u32 {
		self.count
	}

	pub fn borrow_data(&self) -> &Vec<f32> {
		&self.data
	}
}

// @TODO: Fix me.
pub struct AttributeManager {
	count: usize
}

impl AttributeManager {
	pub fn new() -> Self {
		AttributeManager {
			count: 0
		}
	}

	pub fn create(&mut self, data: Vec<f32>, item_size: u32) -> Attribute {
		let attribute = Attribute::new(self.count, data, item_size);
		self.count += 1;
		attribute
	}
}
