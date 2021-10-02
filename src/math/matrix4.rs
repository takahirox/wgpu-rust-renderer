const ELEMENT_NUM: usize = 16;
type Elements = [f32; ELEMENT_NUM];

pub struct Matrix4 {
}

impl Matrix4 {
	pub fn create() -> Elements {
		let mut elements = [0.0; ELEMENT_NUM];
		Matrix4::identity(&mut elements);
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
		m[12] = 0.0;
		m[13] = 0.0;
		m[14] = 0.0;
		m[15] = 1.0;
		m
	}

	pub fn compose<'a>(
		m: &'a mut Elements,
		position: &'a [f32; 3],
		quaternion: &'a [f32; 4],
		scale: &'a [f32; 3],
	) -> &'a mut Elements {
		let x = quaternion[0];
		let y = quaternion[1];
		let z = quaternion[2];
		let w = quaternion[3];

		let x2 = x + x;
		let y2 = y + y;
		let z2 = z + z;

		let xx = x * x2;
		let xy = x * y2;
		let xz = x * z2;

		let yy = y * y2;
		let yz = y * z2;
		let zz = z * z2;

		let wx = w * x2;
		let wy = w * y2;
		let wz = w * z2;

		let sx = scale[0];
		let sy = scale[1];
		let sz = scale[2];

		m[0] = (1.0 - (yy + zz)) * sx;
		m[1] = (xy + wz) * sx;
		m[2] = (xz - wy) * sx;
		m[3] = 0.0;

		m[4] = (xy - wz) * sy;
		m[5] = (1.0 - (xx + zz)) * sy;
		m[6] = (yz + wx) * sy;
		m[7] = 0.0;

		m[8] = (xz + wy) * sz;
		m[9] = (yz - wx) * sz;
		m[10] = (1.0 - (xx + yy)) * sz;
		m[11] = 0.0;

		m[12] = position[0];
		m[13] = position[1];
		m[14] = position[2];
		m[15] = 1.0;

		m
	}
}
