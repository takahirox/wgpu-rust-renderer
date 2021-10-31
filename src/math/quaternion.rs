const ELEMENT_NUM: usize = 4;
type Elements = [f32; ELEMENT_NUM];

pub struct Quaternion {
}

impl Quaternion {
	pub fn create() -> Elements {
		[0.0, 0.0, 0.0, 1.0]
	}

	pub fn set_from_euler<'a>(q: &'a mut Elements, e: &'a [f32; 3]) -> &'a mut Elements {
		// Assume XYZ order
		let x = e[0];
		let y = e[1];
		let z = e[2];

		let c1 = (x / 2.0).cos();
		let c2 = (y / 2.0).cos();
		let c3 = (z / 2.0).cos();

		let s1 = (x / 2.0).sin();
		let s2 = (y / 2.0).sin();
		let s3 = (z / 2.0).sin();

		q[0] = s1 * c2 * c3 + c1 * s2 * s3;
		q[1] = c1 * s2 * c3 - s1 * c2 * s3;
		q[2] = c1 * c2 * s3 + s1 * s2 * c3;
		q[3] = c1 * c2 * c3 - s1 * s2 * s3;

		q
	}

	pub fn set_from_rotation_matrix<'a>(
		q: &'a mut Elements,
		m: &'a [f32; 16],
	) -> &'a mut Elements {
		let m11 = m[0];
		let m12 = m[4];
		let m13 = m[8];
		let m21 = m[1];
		let m22 = m[5];
		let m23 = m[9];
		let m31 = m[2];
		let m32 = m[6];
		let m33 = m[10];

		let trace = m11 + m22 + m33;

		if trace > 0.0 {
			let s = 0.5 / (trace + 1.0).sqrt();
			q[0] = (m32 - m23) * s;
			q[1] = (m13 - m31) * s;
			q[2] = (m21 - m12) * s;
			q[3] = 0.25 / s;
		} else if m11 > m22 && m11 > m33 {
			let s = 2.0 * (1.0 + m11 - m22 - m33).sqrt();
			q[0] = 0.25 * s;
			q[1] = (m12 + m21) / s;
			q[2] = (m13 + m31) / s;
			q[3] = (m32 - m23) / s;
		} else if m22 > m33 {
			let s = 2.0 * (1.0 + m22 - m11 - m33).sqrt();
			q[0] = (m12 + m21) / s;
			q[1] = 0.25 * s;
			q[2] = (m23 + m32) / s;
			q[3] = (m13 - m31) / s;
		} else {
			let s = 2.0 * (1.0 + m33 - m11 - m22).sqrt();
			q[0] = (m13 + m31) / s;
			q[1] = (m23 + m32) / s;
			q[2] = 0.25 * s;
			q[3] = (m21 - m12) / s;
		}

		q
	}
}
