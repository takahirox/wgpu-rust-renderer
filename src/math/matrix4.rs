const ELEMENT_NUM: usize = 16;
type Elements = [f32; ELEMENT_NUM];

pub struct Matrix4 {
}

impl Matrix4 {
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
		m[12] = 0.0;
		m[13] = 0.0;
		m[14] = 0.0;
		m[15] = 1.0;
		m
	}

	pub fn copy<'a>(m: &'a mut Elements, src: &'a Elements) -> &'a mut Elements {
		for i in 0..ELEMENT_NUM {
			m[i] = src[i];
		}
		m
	}

	pub fn multiply<'a>(
		m: &'a mut Elements,
		m1: &'a Elements,
		m2: &'a Elements
	) -> &'a mut Elements {
		let a00 = m1[0];
		let a01 = m1[1];
		let a02 = m1[2];
		let a03 = m1[3];
		let a10 = m1[4];
		let a11 = m1[5];
		let a12 = m1[6];
		let a13 = m1[7];
		let a20 = m1[8];
		let a21 = m1[9];
		let a22 = m1[10];
		let a23 = m1[11];
		let a30 = m1[12];
		let a31 = m1[13];
		let a32 = m1[14];
		let a33 = m1[15];

		let b0 = m2[0];
		let b1 = m2[1];
		let b2 = m2[2];
		let b3 = m2[3];
		m[0] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[1] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[2] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[3] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		let b0 = m2[4];
		let b1 = m2[5];
		let b2 = m2[6];
		let b3 = m2[7];
		m[4] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[5] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[6] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[7] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		let b0 = m2[8];
		let b1 = m2[9];
		let b2 = m2[10];
		let b3 = m2[11];
		m[8] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[9] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[10] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[11] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		let b0 = m2[12];
		let b1 = m2[13];
		let b2 = m2[14];
		let b3 = m2[15];
		m[12] = b0 * a00 + b1 * a10 + b2 * a20 + b3 * a30;
		m[13] = b0 * a01 + b1 * a11 + b2 * a21 + b3 * a31;
		m[14] = b0 * a02 + b1 * a12 + b2 * a22 + b3 * a32;
		m[15] = b0 * a03 + b1 * a13 + b2 * a23 + b3 * a33;

		m
	}

	pub fn invert(m: &mut Elements) -> &mut Elements {
		let a00 = m[0];
		let a01 = m[1];
		let a02 = m[2];
		let a03 = m[3];
		let a10 = m[4];
		let a11 = m[5];
		let a12 = m[6];
		let a13 = m[7];
		let a20 = m[8];
		let a21 = m[9];
		let a22 = m[10];
		let a23 = m[11];
		let a30 = m[12];
		let a31 = m[13];
		let a32 = m[14];
		let a33 = m[15];

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
			// @TODO: Through error?
			return m;
		}

		let det = 1.0 / det;
		m[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
		m[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
		m[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
		m[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
		m[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
		m[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
		m[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
		m[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
		m[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
		m[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
		m[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
		m[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
		m[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
		m[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
		m[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
		m[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;

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

	pub fn make_perspective(
		m: &mut Elements,
		fovy: f32,
		aspect: f32,
		near: f32,
		far: f32
	) -> &mut Elements {
		let f = 1.0 / (fovy / 2.0).tan();
		m[0] = f / aspect;
		m[1] = 0.0;
		m[2] = 0.0;
		m[3] = 0.0;
		m[4] = 0.0;
		m[5] = f;
		m[6] = 0.0;
		m[7] = 0.0;
		m[8] = 0.0;
		m[9] = 0.0;
		m[11] = -1.0;
		m[12] = 0.0;
		m[13] = 0.0;
		m[15] = 0.0;

		let nf = 1.0 / (near - far);
		m[10] = far * nf;
		m[14] = far * near * nf;

		m
	}
}
