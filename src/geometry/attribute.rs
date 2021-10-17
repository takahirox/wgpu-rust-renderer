use uuid::Uuid;

pub struct Attribute {
	count: u32,
	data: Vec<f32>,
	id: Uuid,
	_item_size: u32,
}

impl Attribute {
	pub fn new(data: Vec<f32>, item_size: u32) -> Self {
		Attribute {
			count: data.len() as u32 / item_size,
			data: data,
			id: Uuid::new_v4(),
			_item_size: item_size,
		}
	}

	pub fn get_id(&self) -> Uuid {
		self.id
	}

	pub fn get_count(&self) -> u32 {
		self.count
	}

	pub fn borrow_data(&self) -> &Vec<f32> {
		&self.data
	}
}
