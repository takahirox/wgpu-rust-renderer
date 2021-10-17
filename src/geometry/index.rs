use uuid::Uuid;

// @TODO: Should we reuse Attribute?

pub struct Index {
	count: u32,
	data: Vec<u16>,
	id: Uuid,
}

impl Index {
	pub fn new(data: Vec<u16>) -> Self {
		Index {
			count: data.len() as u32,
			data: data,
			id: Uuid::new_v4(),
		}
	}

	pub fn get_id(&self) -> Uuid {
		self.id
	}

	pub fn get_count(&self) -> u32 {
		self.count
	}

	pub fn borrow_data(&self) -> &Vec<u16> {
		&self.data
	}
}
