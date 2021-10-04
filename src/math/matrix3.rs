const ELEMENT_NUM: usize = 9;
type Elements = [f32; ELEMENT_NUM];

pub struct Matrix3 {
}

impl Matrix3 {
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
		m[4] = 1.0;
		m[5] = 0.0;
		m[6] = 0.0;
		m[7] = 0.0;
		m[8] = 1.0;
		m
	}

	pub fn copy<'a>(m: &'a mut Elements, src: &'a Elements) -> &'a mut Elements {
		for i in 0..ELEMENT_NUM {
			m[i] = src[i];
		}
		m
	}

	pub fn make_normal_from_matrix4<'a>(
		m: &'a mut Elements,
		src: &'a [f32; 16],
	) -> &'a mut Elements {
		let a00 = src[0];
		let a01 = src[1];
		let a02 = src[2];
		let a03 = src[3];
		let a10 = src[4];
		let a11 = src[5];
		let a12 = src[6];
		let a13 = src[7];
		let a20 = src[8];
		let a21 = src[9];
		let a22 = src[10];
		let a23 = src[11];
		let a30 = src[12];
		let a31 = src[13];
		let a32 = src[14];
		let a33 = src[15];

		let b00 = a00 * a11 - a01 * a10;
		let b01 = a00 * a12 - a02 * a10;
		let b02 = a00 * a13 - a03 * a10;
		let b03 = a01 * a12 - a02 * a11;
		let b04 = a01 * a13 - a03 * a11;
		let b05 = a02 * a13 - a03 * a12;
		let b06 = a20 * a31 - a21 * a30;
		let b07 = a20 * a32 - a22 * a30;
		let b08 = a20 * a33 - a23 * a30;
		let b09 = a21 * a32 - a22 * a31;
		let b10 = a21 * a33 - a23 * a31;
		let b11 = a22 * a33 - a23 * a32;

		let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

		if det == 0.0 {
			// @TODO: Error handling?
			return m;
		}

		let det = 1.0 / det;
		m[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
		m[1] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
		m[2] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
		m[3] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
		m[4] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
		m[5] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
		m[6] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
		m[7] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
		m[8] = (a30 * b04 - a31 * b02 + a33 * b00) * det;

		m
	}
}
