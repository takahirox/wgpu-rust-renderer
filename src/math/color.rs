const ELEMENT_NUM: usize = 3;
type Elements = [f32; ELEMENT_NUM];

pub struct Color {
}

impl Color {
	pub fn create() -> Elements {
		[0.0; ELEMENT_NUM]
	}

	pub fn set(c: &mut Elements, r: f32, g: f32, b: f32) -> &mut Elements {
		c[0] = r;
		c[1] = g;
		c[2] = b;
		c
	}

	pub fn copy<'a>(c: &'a mut Elements, c2: &'a Elements) -> &'a mut Elements {
		for i in 0..ELEMENT_NUM {
			c[i] = c2[i];
		}
		c
	}
}
