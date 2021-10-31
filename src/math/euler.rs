const ELEMENT_NUM: usize = 3;
type Elements = [f32; ELEMENT_NUM];

pub struct Euler {
}

impl Euler {
	pub fn create() -> Elements {
		[0.0; ELEMENT_NUM]
	}

	pub fn set_from_quaternion<'a>(
		e: &'a mut Elements,
		q: &'a [f32; 4],
	) -> &'a mut Elements {
		// Assume XYZ order
		let q0 = q[0];
		let q1 = q[1];
		let q2 = q[2];
		let q3 = q[3];

		let q0q0 = q0 * q0;
		let q0q1 = q0 * q1;
		let q0q2 = q0 * q2;
		let q0q3 = q0 * q3;
		let q1q1 = q1 * q1;
		let q1q2 = q1 * q2;
		let q1q3 = q1 * q3;
		let q2q2 = q2 * q2;
		let q2q3 = q2 * q3;
		let q3q3 = q3 * q3;

		let roll = (2.0 * (q2q3 + q0q1)).atan2(q0q0 - q1q1 - q2q2 + q3q3);
		let pitch = (2.0 * (q0q2 - q1q3)).asin();
		let yaw = (2.0 * (q1q2 + q0q3)).atan2(q0q0 + q1q1 - q2q2 - q3q3);

		e[0] = roll;
		e[1] = pitch;
		e[2] = yaw;

		e
	}
}
