const ELEMENT_NUM: usize = 3;
type Elements = [f32; ELEMENT_NUM];

pub struct Vector3 {
}

impl Vector3 {
	pub fn create() -> Elements {
		[0.0; ELEMENT_NUM]
	}

	pub fn set(v: &mut Elements, x: f32, y: f32, z: f32) -> &mut Elements {
		v[0] = x;
		v[1] = y;
		v[2] = z;
		v
	}

	pub fn copy<'a>(v: &'a mut Elements, v2: &'a Elements) -> &'a mut Elements {
		for i in 0..ELEMENT_NUM {
			v[i] = v2[i];
		}
		v
	}

	pub fn add<'a>(v: &'a mut Elements, v2: &'a Elements) -> &'a mut Elements {
		for i in 0..ELEMENT_NUM {
			v[i] += v2[i];
		}
		v
	}

	pub fn sub<'a>(v: &'a mut Elements, v2: &'a Elements) -> &'a mut Elements {
		for i in 0..ELEMENT_NUM {
			v[i] -= v2[i];
		}
		v
	}

	pub fn length(v: &Elements) -> f32 {
		(v[0].powf(2.0) + v[1].powf(2.0) + v[2].powf(2.0)).sqrt()
	}

	pub fn normalize(v: &mut Elements) -> &mut Elements {
		let length = Self::length(v);

		// @TODO: Error handling?
		if length != 0.0 {
			for i in 0..ELEMENT_NUM {
				v[i] /= length;
			}
		}

		v
	}
}
