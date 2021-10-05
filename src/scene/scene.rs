use std::collections::HashMap;

use crate::{
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		object::Object,
	},
	math::color::Color,
};

struct ComponentManager<T> {
	components: Vec<T>,
	object_id_map: HashMap<usize, usize>, // object id -> index in components
	object_ids: Vec<usize>, // object ids. Same order with components
}

impl<T> ComponentManager<T> {
	fn new() -> Self {
		ComponentManager {
			components: Vec::new(),
			object_id_map: HashMap::new(),
			object_ids: Vec::new(),
		}
	}

	fn _empty(&self) -> bool {
		self.components.len() == 0
	}

	fn has(&self, object_id: usize) -> bool {
		self.object_id_map.contains_key(&object_id)
	}

	fn add(&mut self, object_id: usize, component: T) -> &mut Self {
		if self.has(object_id) {
			// @TODO: Error handling?
			return self;
		}
		self.object_id_map.insert(object_id, self.components.len());
		self.components.push(component);
		self.object_ids.push(object_id);
		self
	}

	fn borrow(&self, object_id: usize) -> Option<&T> {
		match self.has(object_id) {
			true => {
				let index = *self.object_id_map.get(&object_id).unwrap();
				Some(&self.components[index])
			},
			false => None,
		}
	}

	fn borrow_mut(&mut self, object_id: usize) -> Option<&mut T> {
		match self.has(object_id) {
			true => {
				let index = *self.object_id_map.get(&object_id).unwrap();
				Some(&mut self.components[index])
			},
			false => None,
		}
	}
}

// @TODO: component_manager_map: HashMap<TypeID: ComponentManager<T>> ?
pub struct Scene {
	active_camera_id: Option<usize>,
	background_color: [f32; 3],
	objects: Vec<Object>,
	camera_manager: ComponentManager<PerspectiveCamera>,
	mesh_manager: ComponentManager<Mesh>,
}

impl Scene {
	pub fn new() -> Self {
		Scene {
			active_camera_id: None,
			background_color: *Color::set(&mut Color::create(), 1.0, 1.0, 1.0),
			camera_manager: ComponentManager::<PerspectiveCamera>::new(),
			mesh_manager: ComponentManager::<Mesh>::new(),
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
		if self.mesh_manager.has(id) {
			// @TODO: Error handling?
			return self;
		}
		self.mesh_manager.add(id, mesh);
		self
	}

	pub fn borrow_mesh(&self, id: usize) -> Option<&Mesh> {
		self.mesh_manager.borrow(id)
	}

	pub fn borrow_mesh_mut(&mut self, id: usize) -> Option<&mut Mesh> {
		self.mesh_manager.borrow_mut(id)
	}

	pub fn borrow_background_color(&self) -> &[f32; 3] {
		&self.background_color
	}

	pub fn borrow_background_color_mut(&mut self) -> &[f32; 3] {
		&mut self.background_color
	}

	pub fn add_camera(&mut self, id: usize, camera: PerspectiveCamera) -> &mut Self {
		if self.camera_manager.has(id) {
			// @TODO: Error handling?
			return self;
		}
		self.camera_manager.add(id, camera);
		self
	}

	pub fn borrow_camera(&self, id: usize) -> Option<&PerspectiveCamera> {
		self.camera_manager.borrow(id)
	}

	pub fn borrow_camera_mut(&mut self, id: usize) -> Option<&mut PerspectiveCamera> {
		self.camera_manager.borrow_mut(id)
	}

	pub fn set_active_camera_id(&mut self, id: usize) {
		if !self.camera_manager.has(id) {
			// @TODO: Error handling
			return;
		}
		self.active_camera_id = Some(id);
	}

	pub fn get_active_camera_id(&self) -> Option<usize> {
		self.active_camera_id
	}

	pub fn borrow_active_camera(&self) -> Option<&PerspectiveCamera> {
		if let Some(active_camera_id) = self.active_camera_id {
			self.borrow_camera(active_camera_id)
		} else {
			None
		}
	}

	pub fn borrow_active_camera_mut(&mut self) -> Option<&mut PerspectiveCamera> {
		if let Some(active_camera_id) = self.active_camera_id {
			self.borrow_camera_mut(active_camera_id)
		} else {
			None
		}
	}

	pub fn update_matrices(&mut self) {
		for object in self.objects.iter_mut() {
			object.update_matrix();
		}
	}
}
