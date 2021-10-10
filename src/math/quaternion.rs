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
}
