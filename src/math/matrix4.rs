use crate::math::{
	quaternion::Quaternion,
	vector3::Vector3,
};

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

	// Good name?
	pub fn set_from_2d_array<'a>(m: &'a mut Elements, src: &'a [[f32; 4]; 4]) -> &'a mut Elements {
		for i in 0..4 {
			for j in 0..4 {
				m[i * 4 + j] = src[j][i];
			}
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

	pub fn decompose<'a>(
		position: &mut [f32; 3],
		quaternion: &mut [f32; 4],
		scale: &mut [f32; 3],
		m: &Elements,
	) {
		let mut v = Vector3::create();
		let sx = Vector3::length(Vector3::set(&mut v, m[0], m[1], m[2]));
		let sy = Vector3::length(Vector3::set(&mut v, m[4], m[5], m[6]));
		let sz = Vector3::length(Vector3::set(&mut v, m[8], m[9], m[10]));

		let sx = match Matrix4::determinant(m) < 0.0 {
			true => -sx,
			false => sx,
		};

		position[0] = m[12];
		position[1] = m[13];
		position[2] = m[14];

		let inv_sx = 1.0 / sx;
		let inv_sy = 1.0 / sy;
		let inv_sz = 1.0 / sz;

		let mut m2 = Self::create();
		Self::copy(&mut m2, m);

		m2[0] *= inv_sx;
		m2[1] *= inv_sx;
		m2[2] *= inv_sx;

		m2[4] *= inv_sy;
		m2[5] *= inv_sy;
		m2[6] *= inv_sy;

		m2[8] *= inv_sz;
		m2[9] *= inv_sz;
		m2[10] *= inv_sz;

		Quaternion::set_from_rotation_matrix(quaternion, &m2);

		scale[0] = sx;
		scale[1] = sy;
		scale[2] = sz;
	}

	pub fn determinant(m: &Elements) -> f32 {
		let n11 = m[0];
		let n12 = m[4];
		let n13 = m[8];
		let n14 = m[12];
		let n21 = m[1];
		let n22 = m[5];
		let n23 = m[9];
		let n24 = m[13];
		let n31 = m[2];
		let n32 = m[6];
		let n33 = m[10];
		let n34 = m[14];
		let n41 = m[3];
		let n42 = m[7];
		let n43 = m[11];
		let n44 = m[15];

		n41 * (
			n14 * n23 * n32
			- n13 * n24 * n32
			- n14 * n22 * n33
			+ n12 * n24 * n33
			+ n13 * n22 * n34
			- n12 * n23 * n34
		) +
		n42 * (
			n11 * n23 * n34
			- n11 * n24 * n33
			+ n14 * n21 * n33
			- n13 * n21 * n34
			+ n13 * n24 * n31
			- n14 * n23 * n31
		) +
		n43 * (
			n11 * n24 * n32
			- n11 * n22 * n34
			- n14 * n21 * n32
			+ n12 * n21 * n34
			+ n14 * n22 * n31
			- n12 * n24 * n31
		) +
		n44 * (
			n13 * n22 * n31
			- n11 * n23 * n32
			+ n11 * n22 * n33
			+ n13 * n21 * n32
			- n12 * n21 * n33
			+ n12 * n23 * n31
		)
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
