use crate::{
	geometry::geometry::Geometry,
	material::material::Material,
	resource::resource::ResourceId,
};

// @TODO: Support shared geometry and material
pub struct Mesh {
	geometry: ResourceId<Geometry>,
	material: ResourceId<Material>,
}

impl Mesh {
	pub fn new(
		geometry: ResourceId<Geometry>,
		material: ResourceId<Material>,
	) -> Self {
		Mesh {
			geometry: geometry,
			material: material,
		}
	}

	pub fn borrow_geometry(&self) -> &ResourceId<Geometry> {
		&self.geometry
	}

	pub fn borrow_material(&self) -> &ResourceId<Material> {
		&self.material
	}
}
