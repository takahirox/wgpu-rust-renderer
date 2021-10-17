use std::collections::HashMap;
use uuid::Uuid;

use crate::{
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		node::Node,
	},
	math::color::Color,
};

struct ComponentManager<T> {
	components: Vec<T>,
	node_id_map: HashMap<usize, usize>, // node id -> index in components
	node_ids: Vec<usize>, // node ids. Same order with components
}

impl<T> ComponentManager<T> {
	fn new() -> Self {
		ComponentManager {
			components: Vec::new(),
			node_id_map: HashMap::new(),
			node_ids: Vec::new(),
		}
	}

	fn _empty(&self) -> bool {
		self.components.len() == 0
	}

	fn has(&self, node_id: usize) -> bool {
		self.node_id_map.contains_key(&node_id)
	}

	fn add(&mut self, node_id: usize, component: T) -> &mut Self {
		if self.has(node_id) {
			// @TODO: Error handling?
			return self;
		}
		self.node_id_map.insert(node_id, self.components.len());
		self.components.push(component);
		self.node_ids.push(node_id);
		self
	}

	fn borrow(&self, node_id: usize) -> Option<&T> {
		match self.has(node_id) {
			true => {
				let index = *self.node_id_map.get(&node_id).unwrap();
				Some(&self.components[index])
			},
			false => None,
		}
	}

	fn borrow_mut(&mut self, node_id: usize) -> Option<&mut T> {
		match self.has(node_id) {
			true => {
				let index = *self.node_id_map.get(&node_id).unwrap();
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
	nodes: Vec<Node>,
	node_index_map: HashMap<Uuid, usize>, // node id -> index in nodes
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
			nodes: Vec::new(),
			node_index_map: HashMap::new(),
		}
	}

	pub fn create_node(&mut self) -> usize {
		let node = Node::new();
		self.node_index_map.insert(node.get_id(), self.nodes.len());
		self.nodes.push(node);
		self.nodes.len() - 1
	}

	pub fn get_nodes_num(&self) -> usize {
		self.nodes.len()
	}

	pub fn borrow_node(&self, id: usize) -> Option<&Node> {
		if id >= self.nodes.len() {
			return None;
		}
		Some(&self.nodes[id])
	}

	pub fn borrow_node_mut(&mut self, id: usize) -> Option<&mut Node> {
		if id >= self.nodes.len() {
			return None;
		}
		Some(&mut self.nodes[id])
	}

	pub fn add_mesh(&mut self, id: usize, mesh: Mesh) -> &mut Self {
		if self.mesh_manager.has(id) {
			// @TODO: Error handling?
			return self;
		}
		self.mesh_manager.add(id, mesh);
		self
	}

	pub fn borrow_mesh(&self, node: &Node) -> Option<&Mesh> {
		let id = self.node_index_map.get(&node.get_id()).unwrap();
		self.mesh_manager.borrow(*id)
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
		for node in self.nodes.iter_mut() {
			node.update_matrix();
		}
	}
}
