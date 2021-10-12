// @TODO: Should we reuse Attribute?

pub struct Index {
	count: u32,
	data: Vec<u16>,
	id: usize,
}

impl Index {
	fn new(id: usize, data: Vec<u16>) -> Self {
		Index {
			count: data.len() as u32,
			data: data,
			id: id,
		}
	}

	pub fn get_id(&self) -> usize {
		self.id
	}

	pub fn get_count(&self) -> u32 {
		self.count
	}

	pub fn borrow_data(&self) -> &Vec<u16> {
		&self.data
	}
}

// @TODO: Fix me.
pub struct IndexManager {
	count: usize
}

impl IndexManager {
	pub fn new() -> Self {
		IndexManager {
			count: 0
		}
	}

	pub fn create(&mut self, data: Vec<u16>) -> Index {
		let index = Index::new(self.count, data);
		self.count += 1;
		index
	}
}
