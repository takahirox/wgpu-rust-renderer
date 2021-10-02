use std::collections::HashMap;

use crate::scene::object::Object;
use crate::scene::mesh::Mesh;

pub struct Scene {
	mesh_object_id_map: HashMap<usize, usize>, // object id -> index in meshes
	mesh_object_ids: Vec<usize>, // object ids. Same order with meshes.
	meshes: Vec<Mesh>,
	objects: Vec<Object>,
}

impl Scene {
	pub fn new() -> Self {
		Scene {
			mesh_object_id_map: HashMap::new(),
			mesh_object_ids: Vec::new(),
			meshes: Vec::new(),
			objects: Vec::new(),
		}
	}

	pub fn create_object(&mut self) -> usize {
		let object = Object::new(self.objects.len());
		self.objects.push(object);
		self.objects.len() - 1
	}

	pub fn get_objects_num(&self) -> usize {
		self.objects.len()
	}

	pub fn borrow_object(&self, id: usize) -> Option<&Object> {
		if id >= self.objects.len() {
			return None;
		}
		Some(&self.objects[id])
	}

	pub fn borrow_object_mut(&mut self, id: usize) -> Option<&mut Object> {
		if id >= self.objects.len() {
			return None;
		}
		Some(&mut self.objects[id])
	}

	pub fn add_mesh(&mut self, id: usize, mesh: Mesh) -> &mut Self {
		if self.mesh_object_id_map.contains_key(&id) {
			// @TODO: Error handling?
			return self;
		}
		self.mesh_object_id_map.insert(id, self.meshes.len());
		self.meshes.push(mesh);
		self.mesh_object_ids.push(id);
		self
	}

	pub fn borrow_mesh(&self, id: usize) -> Option<&Mesh> {
		if !self.mesh_object_id_map.contains_key(&id) {
			return None;
		}
		Some(&self.meshes[*self.mesh_object_id_map.get(&id).unwrap()])
	}
}

