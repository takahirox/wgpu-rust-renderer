use std::{
	any::{
		Any,
		TypeId,
	},
	collections::HashMap,
};

use crate::{
	math::color::Color,
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		node::{
			Node,
			NodeExecutor,
		},
	},
};


trait ResourceLinksTrait {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ResourceLinks<T1, T2> {
	links: HashMap<ResourceId<T1>, ResourceId<T2>>,
}

impl<T1: 'static, T2: 'static> ResourceLinksTrait for ResourceLinks<T1, T2> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T1: 'static, T2: 'static> ResourceLinks<T1, T2> {
	pub fn new() -> Self {
		ResourceLinks {
			links: HashMap::new(),
		}
	}

	pub fn has(&self, rid_from: &ResourceId<T1>) -> bool {
		self.links.contains_key(rid_from)
	}

	pub fn add(&mut self, rid_from: &ResourceId<T1>, rid_to: &ResourceId<T2>) {
		self.links.insert(*rid_from, *rid_to);
	}

	pub fn borrow(&self, rid_from: &ResourceId<T1>) -> Option<&ResourceId<T2>> {
		self.links.get(rid_from)
	}
}

fn cast_links<T1: 'static, T2: 'static>(links: &dyn ResourceLinksTrait) -> &ResourceLinks<T1, T2> {
	links
		.as_any()
		.downcast_ref::<ResourceLinks<T1, T2>>()
		.unwrap()
}

fn cast_links_mut<T1: 'static, T2: 'static>(links: &mut dyn ResourceLinksTrait) -> &mut ResourceLinks<T1, T2> {
	links
		.as_any_mut()
		.downcast_mut::<ResourceLinks<T1, T2>>()
		.unwrap()
}

pub struct Scene {
	background_color: [f32; 3],
	links: HashMap<TypeId, Box<dyn ResourceLinksTrait>>,
	nodes: Vec<ResourceId<Node>>,
}

impl Scene {
	pub fn new() -> Self {
		let mut links = HashMap::new();
		Self::add_links::<Node, Mesh>(&mut links);
		Self::add_links::<Mesh, Node>(&mut links);
		Self::add_links::<Node, PerspectiveCamera>(&mut links);
		Self::add_links::<PerspectiveCamera, Node>(&mut links);

		Scene {
			background_color: *Color::set(&mut Color::create(), 1.0, 1.0, 1.0),
			links: links,
			nodes: Vec::new(),
		}
	}

	fn add_links<T1: 'static, T2: 'static>(links: &mut HashMap<TypeId, Box<dyn ResourceLinksTrait>>) {
		links.insert(TypeId::of::<(T1, T2)>(), Box::new(ResourceLinks::<T1, T2>::new()));
	}

	fn borrow_links<T1: 'static, T2: 'static>(&self) -> &ResourceLinks<T1, T2> {
		if let Some(links) = self.links.get(&TypeId::of::<(T1, T2)>()) {
			cast_links(links.as_ref())
		} else {
			// @TODO: Proper error handling
			panic!("Unknown Type");
		}
	}

	fn borrow_links_mut<T1: 'static, T2: 'static>(&mut self) -> &mut ResourceLinks<T1, T2> {
		if let Some(links) = self.links.get_mut(&TypeId::of::<(T1, T2)>()) {
			cast_links_mut(links.as_mut())
		} else {
			// @TODO: Proper error handling
			panic!("Unknown Type");
		}
	}

	// Where should be this method placed?
	pub fn assign<T: 'static>(
		&mut self,
		rid1: &ResourceId<Node>,
		rid2: &ResourceId<T>,
	) {
		self.borrow_links_mut::<Node, T>().add(rid1, rid2);
		self.borrow_links_mut::<T, Node>().add(rid2, rid1);
	}

	// @TODO: Rename?
	pub fn borrow_assigned_from<T: 'static>(&self, rid: &ResourceId<T>) -> Option<&ResourceId<Node>> {
		self.borrow_links::<T, Node>().borrow(rid)
	}

	pub fn borrow_assigned_to<T: 'static>(&self, rid: &ResourceId<Node>) -> Option<&ResourceId<T>> {
		self.borrow_links::<Node, T>().borrow(rid)
	}

	pub fn add_node(&mut self, rid: &ResourceId<Node>) {
		self.nodes.push(*rid);
	}

	pub fn collect_nodes(&self, pools: &ResourcePools) -> Vec<ResourceId<Node>> {
		let mut nodes = Vec::new();
		let pool = pools.borrow::<Node>();
		for node in self.nodes.iter() {
			NodeExecutor::collect_nodes(pool, node, &mut nodes);
		}
		nodes
	}

	pub fn borrow_background_color(&self) -> &[f32; 3] {
		&self.background_color
	}

	pub fn borrow_background_color_mut(&mut self) -> &[f32; 3] {
		&mut self.background_color
	}

	pub fn update_matrices(&self, pools: &ResourcePools) {
		// @TODO: Write comment about why unsafe
		// @TODO: Remove unsafe
		let pool = pools.borrow_mut_unsafe::<Node>();
		for node in self.nodes.iter() {
			NodeExecutor::update_matrices(pool, node);
		}
	}
}
