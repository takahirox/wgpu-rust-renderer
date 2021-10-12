use crate::material::material::Material;
use crate::geometry::geometry::Geometry;

// @TODO: Support shared geometry and material
pub struct Mesh {
	geometry: Geometry,
	material: Material,
}

impl Mesh {
	pub fn new(geometry: Geometry, material: Material) -> Self {
		Mesh {
			geometry: geometry,
			material: material,
		}
	}

	pub fn borrow_geometry(&self) -> &Geometry {
		&self.geometry
	}

	pub fn borrow_material(&self) -> &Material {
		&self.material
	}
}
