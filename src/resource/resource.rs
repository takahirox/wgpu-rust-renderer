use std::{
	any::{
		Any,
		TypeId,
	},
	collections::HashMap,
	hash::{
		Hash,
		Hasher,
	},
	marker::PhantomData,
	mem::transmute,
};

use crate::{
	geometry::{
		attribute::Attribute,
		geometry::Geometry,
		index::Index,
	},
	material::{
		material::Material,
		node::node::MaterialNode,
	},
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		node::Node,
		scene::Scene,
	},
	texture::{
		sampler::Sampler,
		texture::Texture,
	},
};

trait ResourcePoolTrait {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ResourcePool<T> {
	resources: Vec<T>,
}

impl<T: 'static> ResourcePoolTrait for ResourcePool<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}	
}

fn cast_pool<T: 'static>(pool: &dyn ResourcePoolTrait) -> &ResourcePool<T> {
	pool
		.as_any()
		.downcast_ref::<ResourcePool<T>>()
		.unwrap()
}

fn cast_pool_mut<T: 'static>(pool: &mut dyn ResourcePoolTrait) -> &mut ResourcePool<T> {
	pool
		.as_any_mut()
		.downcast_mut::<ResourcePool<T>>()
		.unwrap()
}

// @TODO: Write comment
fn cast_pool_mut_unsafe<T: 'static>(pool: &Box<dyn ResourcePoolTrait>) -> &mut ResourcePool<T> {
	let ptr = cast_pool(pool.as_ref())
		as *const ResourcePool<T> as *mut ResourcePool<T>;
	unsafe { transmute(ptr) }
}

impl<T: 'static> ResourcePool<T> {
	pub fn new() -> Self {
		ResourcePool {
			resources: Vec::new(),
		}
	}

	pub fn add(&mut self, resource: T) -> ResourceId<T> {
		let id = self.resources.len();
		self.resources.push(resource);
		ResourceId::new(id)
	}

	pub fn borrow(&self, r_id: &ResourceId<T>) -> Option<&T> {
		if r_id.id < self.resources.len() {
			Some(&self.resources[r_id.id])
		} else {
			None
		}
	}

	pub fn borrow_mut(&mut self, r_id: &ResourceId<T>) -> Option<&mut T> {
		if r_id.id < self.resources.len() {
			Some(&mut self.resources[r_id.id])
		} else {
			None
		}
	}
}

pub struct ResourcePools {
	pools: HashMap<TypeId, Box<dyn ResourcePoolTrait>>,
}

impl ResourcePools {
	pub fn new() -> Self {
		let mut pools = HashMap::new();
		Self::add::<Attribute>(&mut pools);
		Self::add::<Geometry>(&mut pools);
		Self::add::<Index>(&mut pools);
		Self::add::<Material>(&mut pools);
		Self::add::<Box<dyn MaterialNode>>(&mut pools);
		Self::add::<Mesh>(&mut pools);
		Self::add::<Node>(&mut pools);
		Self::add::<PerspectiveCamera>(&mut pools);
		Self::add::<Scene>(&mut pools);
		Self::add::<Sampler>(&mut pools);
		Self::add::<Texture>(&mut pools);

		ResourcePools {
			pools: pools,
		}
	}

	fn add<T: 'static>(pools: &mut HashMap<TypeId, Box<dyn ResourcePoolTrait>>) {
		pools.insert(TypeId::of::<T>(), Box::new(ResourcePool::<T>::new()));
	}

	pub fn borrow<T: 'static>(&self) -> &ResourcePool<T> {
		if let Some(pool) = self.pools.get(&TypeId::of::<T>()) {
			cast_pool(pool.as_ref())
		} else {
			// @TODO: Proper error handling
			// @TODO: Trait bound
			panic!("Unknown Type");
		}
	}

	pub fn borrow_mut<T: 'static>(&mut self) -> &mut ResourcePool<T> {
		if let Some(pool) = self.pools.get_mut(&TypeId::of::<T>()) {
			cast_pool_mut(pool.as_mut())
		} else {
			// @TODO: Proper error handling
			panic!("Unknown Type");
		}
	}

	// @TODO: Write comment
	pub fn borrow_mut_unsafe<T: 'static>(&self) -> &mut ResourcePool<T> {
		if let Some(pool) = self.pools.get(&TypeId::of::<T>()) {
			cast_pool_mut_unsafe(pool)
		} else {
			// @TODO: Proper error handling
			// @TODO: Trait bound
			panic!("Unknown Type");
		}
	}
}

pub struct ResourceId<T> {
	pub id: usize,
	_phantom: PhantomData<T>,
}

impl<T> ResourceId<T> {
	fn new(id: usize) -> Self {
		ResourceId {
			id: id,
			_phantom: PhantomData
		}
	}
}

impl<T> Copy for ResourceId<T> {
}

impl<T> Clone for ResourceId<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T> Hash for ResourceId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialEq for ResourceId<T> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl<T> Eq for ResourceId<T> {
}
