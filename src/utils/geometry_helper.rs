use crate::{
	math::vector3::Vector3,
	scene::{
		attribute::AttributeManager,
		geometry::Geometry,
		index::IndexManager,
	},
};

pub struct GeometryHelper {
}

impl GeometryHelper {
	pub fn create_triangle(
		attribute_manager: &mut AttributeManager,
		index_manager: &mut IndexManager,
		width: f32,
		height: f32,
	) -> Geometry {
		let dy = 0.75_f32.sqrt() / 2.0;
		let positions = [
			0.0, (0.75_f32.sqrt() - dy) * height, 0.0,
			0.5 * width, -dy * height, 0.0,
			-0.5 * width, -dy * height, 0.0,
		].to_vec();

		let normals = [
			0.0, 0.0, 1.0,
			0.0, 0.0, 1.0,
			0.0, 0.0, 1.0,
		].to_vec();

		let uvs = [
			0.5, 0.0,
			1.0, 1.0,
			0.0, 1.0,
		].to_vec();

		let indices = [
			0, 1, 2,
		].to_vec();

		let mut geometry = Geometry::new();
		geometry.set_attribute("position", attribute_manager.create(positions, 3));
		geometry.set_attribute("normal", attribute_manager.create(normals, 3));
		geometry.set_attribute("uv", attribute_manager.create(uvs, 2));
		geometry.set_index(index_manager.create(indices));
		geometry
	}

	pub fn create_plane(
		attribute_manager: &mut AttributeManager,
		index_manager: &mut IndexManager,
		width: f32,
		height: f32,
	) -> Geometry {
		let positions = [
			// top-left
			-0.5 * width, -0.5 * height, 0.0,
			// top-right
			0.5 * width, -0.5 * height, 0.0,
			// bottom-left
			-0.5 * width, 0.5 * height, 0.0,
			// bottom-right
			0.5 * width, 0.5 * height, 0.0,
		].to_vec();

		let normals = [
			// top-left
			0.0, 0.0, 1.0,
			// top-right
			0.0, 0.0, 1.0,
			// bottom-left
			0.0, 0.0, 1.0,
			// bottom-right
			0.0, 0.0, 1.0,
		].to_vec();

		let uvs = [
			// top-left
			0.0, 0.0,
			// top-right
			1.0, 0.0,
			// bottom-left
			0.0, 1.0,
			// bottom-right
			1.0, 1.0,
		].to_vec();

		let indices = [
			0, 1, 2,
			1, 3, 2,
		].to_vec();

		let mut geometry = Geometry::new();
		geometry.set_attribute("position", attribute_manager.create(positions, 3));
		geometry.set_attribute("normal", attribute_manager.create(normals, 3));
		geometry.set_attribute("uv", attribute_manager.create(uvs, 2));
		geometry.set_index(index_manager.create(indices));
		geometry
	}

	pub fn create_box(
		attribute_manager: &mut AttributeManager,
		index_manager: &mut IndexManager,
		width: f32,
		height: f32,
		depth: f32,
	) -> Geometry {
		let mut positions = Vec::new();
		let mut normals = Vec::new();
		let mut uvs = Vec::new();
		let mut indices = Vec::new();

		let mut position_vec = Vector3::create();
		let mut normal_vec = Vector3::create();

		// @TODO: Clean up
		for face in 0..6 {
			let (x, y, z, dx, dy, dz) = match face {
				// front
				0 => (
					-0.5, 0.5, 0.5,
					[0.0, 1.0, 0.0, 1.0],
					[0.0, 0.0, -1.0, -1.0],
					[0.0, 0.0, 0.0, 0.0],
				),
				// right
				1 => (
					0.5, 0.5, 0.5,
					[0.0, 0.0, 0.0, 0.0],
					[0.0, 0.0, -1.0, -1.0],
					[-1.0, 0.0, -1.0, 0.0],
				),
				// back
				2 => (
					0.5, 0.5, -0.5,
					[0.0, -1.0, 0.0, -1.0],
					[0.0, 0.0, -1.0, -1.0],
					[0.0, 0.0, 0.0, 0.0],
				),
				// left
				3 => (
					-0.5, 0.5, -0.5,
					[0.0, 0.0, 0.0, 0.0],
					[0.0, 0.0, -1.0, -1.0],
					[1.0, 0.0, 1.0, 0.0],
				),
				// top
				4 => (
					-0.5, 0.5, -0.5,
					[0.0, 1.0, 0.0, 1.0],
					[0.0, 0.0, 0.0, 0.0],
					[0.0, 0.0, 1.0, 1.0],
				),
				// bottom
				_ => (
					-0.5, -0.5, 0.5,
					[0.0, 1.0, 0.0, 1.0],
					[0.0, 0.0, 0.0, 0.0],
					[0.0, 0.0, -1.0, -1.0],
				),
			};

			for i in 0..4 {
				position_vec[0] = (x + dx[i]) * width;
				position_vec[1] = (y + dy[i]) * height;
				position_vec[2] = (z + dz[i]) * depth;

				Vector3::copy(&mut normal_vec, &position_vec);
				Vector3::normalize(&mut normal_vec);

				for j in 0..3 {
					positions.push(position_vec[j]);
					normals.push(normal_vec[j]);
				}
			}

			uvs.push(0.0);
			uvs.push(0.0);

			uvs.push(1.0);
			uvs.push(0.0);

			uvs.push(0.0);
			uvs.push(1.0);

			uvs.push(1.0);
			uvs.push(1.0);

			indices.push(face * 4 + 0);
			indices.push(face * 4 + 1);
			indices.push(face * 4 + 2);

			indices.push(face * 4 + 1);
			indices.push(face * 4 + 3);
			indices.push(face * 4 + 2);
		}

		let mut geometry = Geometry::new();
		geometry.set_attribute("position", attribute_manager.create(positions, 3));
		geometry.set_attribute("normal", attribute_manager.create(normals, 3));
		geometry.set_attribute("uv", attribute_manager.create(uvs, 2));
		geometry.set_index(index_manager.create(indices));
		geometry
	}
}
