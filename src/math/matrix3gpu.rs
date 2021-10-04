const ELEMENT_NUM: usize = 12;
type Elements = [f32; ELEMENT_NUM];

pub struct Matrix3GPU {
}

impl Matrix3GPU {
	pub fn create() -> Elements {
		let mut elements = [0.0; ELEMENT_NUM];
		Self::identity(&mut elements);
		elements
	}

	pub fn identity(m: &mut Elements) -> &mut Elements {
		m[0] = 1.0;
		m[1] = 0.0;
		m[2] = 0.0;
		m[3] = 0.0;
		m[4] = 0.0;
		m[5] = 1.0;
		m[6] = 0.0;
		m[7] = 0.0;
		m[8] = 0.0;
		m[9] = 0.0;
		m[10] = 1.0;
		m[11] = 0.0;
		m
	}

	pub fn copy<'a>(m: &'a mut Elements, src: &'a Elements) -> &'a mut Elements {
		for i in 0..ELEMENT_NUM {
			m[i] = src[i];
		}
		m
	}

	pub fn copy_from_matrix3<'a>(m: &'a mut Elements, src: &'a [f32; 9]) -> &'a mut Elements {
		// @TODO: Use loop?
		m[0] = src[0];
		m[1] = src[1];
		m[2] = src[2];
		m[3] = 0.0;
		m[4] = src[3];
		m[5] = src[4];
		m[6] = src[5];
		m[7] = 0.0;
		m[8] = src[6];
		m[9] = src[7];
		m[10] = src[8];
		m[11] = 0.0;
		m
	}
}
