use crate::scene::geometry::Geometry;

// @TODO: Support shared geometry
pub struct Mesh {
	geometry: Geometry
}

impl Mesh {
	pub fn new(geometry: Geometry) -> Self {
		Mesh {
			geometry: geometry
		}
	}

	pub fn borrow_geometry(&self) -> &Geometry {
		&self.geometry
	}
}
