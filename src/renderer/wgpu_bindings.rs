use std::collections::HashMap;

use crate::{
	material::{
		material::Material,
		node::node::{
			MaterialNode,
			UniformContents,
		},
	},
	math::{
		matrix3::Matrix3,
		matrix3gpu::Matrix3GPU,
		matrix4::Matrix4,
	},
	renderer::{
		wgpu_samplers::WGPUSamplers,
		wgpu_textures::WGPUTextures,
	},
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	scene::{
		camera::PerspectiveCamera,
		node::Node,
	},
};

pub struct WGPUBinding {
	buffers: Vec<wgpu::Buffer>,
	group: wgpu::BindGroup,
	layout: wgpu::BindGroupLayout,
}

impl WGPUBinding {
	fn new(
		device: &wgpu::Device,
		wgpu_textures: &WGPUTextures,
		wgpu_samplers: &WGPUSamplers,
		pools: &ResourcePools,
		material: &Material,
	) -> Self {
		let layout = Self::build_layout(device, pools, material);

		let textures = material.borrow_textures(
			pools.borrow::<Box<dyn MaterialNode>>(),
		);
		let mut textures_gpu = Vec::new();
		for texture in textures.iter() {
			if let Some(texture) = wgpu_textures.borrow(texture) {
				textures_gpu.push(texture);
			}
		}

		let samplers = material.borrow_samplers(
			pools.borrow::<Box<dyn MaterialNode>>(),
		);
		let mut samplers_gpu = Vec::new();
		for sampler in samplers.iter() {
			if let Some(sampler) = wgpu_samplers.borrow(sampler) {
				samplers_gpu.push(sampler);
			}
		}

		let buffers = Self::build_buffers(device, pools, material);
		let group = Self::build_group(
			device,
			&layout,
			&buffers,
			&textures_gpu,
			&samplers_gpu,
		);

		WGPUBinding {
			buffers: buffers,
			group: group,
			layout: layout,
		}
	}

	pub fn borrow_group(&self) -> &wgpu::BindGroup {
		&self.group
	}

	pub fn borrow_layout(&self) -> &wgpu::BindGroupLayout {
		&self.layout
	}

	pub fn borrow_buffer(&self, index: usize) -> &wgpu::Buffer {
		&self.buffers[index]
	}

	pub fn update(
		&self,
		queue: &wgpu::Queue,
		pools: &ResourcePools,
		node: &Node,
		camera: &PerspectiveCamera,
		camera_node: &Node,
		material: &Material,
	) {
		// @TODO: Is calculating them here inefficient?
		let mut model_view_matrix = Matrix4::create();
		let mut camera_matrix_inverse = Matrix4::create();
		let mut normal_matrix = Matrix3::create();
		let mut normal_matrix_gpu = Matrix3GPU::create();
		Matrix4::copy(&mut camera_matrix_inverse, camera_node.borrow_matrix());
		Matrix4::invert(&mut camera_matrix_inverse);
		Matrix4::multiply(&mut model_view_matrix, &camera_matrix_inverse, node.borrow_matrix());
		Matrix3::make_normal_from_matrix4(&mut normal_matrix, &model_view_matrix);
		Matrix3GPU::copy_from_matrix3(&mut normal_matrix_gpu, &normal_matrix);

		// binding 0 : Object (model-view matrix, normal matrix)
		// binding 1 : Camera (projection matrix)
		// binding 2 : Uniform buffers
		// @TODO: Should we calculate projection matrix * model-view matrix in CPU?
		queue.write_buffer(&self.buffers[0], 0, bytemuck::cast_slice(&model_view_matrix));
		queue.write_buffer(&self.buffers[0], 64, bytemuck::cast_slice(&normal_matrix_gpu));
		queue.write_buffer(&self.buffers[1], 0, bytemuck::cast_slice(camera.borrow_projection_matrix()));

		let mut offset = 0;
		// @TODO: Optimize
		for contents in material.borrow_contents(
			pools.borrow::<Box<dyn MaterialNode>>(),
		).iter() {
			match contents {
				UniformContents::Float {value: _} |
				UniformContents::Vector3 {value: _} |
				UniformContents::Matrix4 {value: _} => {
					let align = get_align(contents);
					offset += (align - (offset % align)) % align;
				},
				_ => {},
			};

			// Can we use generics?
			match contents {
				UniformContents::Float {value} => {
					queue.write_buffer(&self.buffers[2], offset, bytemuck::cast_slice(value));
				},
				UniformContents::Vector3 {value} => {
					queue.write_buffer(&self.buffers[2], offset, bytemuck::cast_slice(value));
				},
				UniformContents::Matrix4 {value} => {
					queue.write_buffer(&self.buffers[2], offset, bytemuck::cast_slice(value));
				},
				_ => {},
			};

			match contents {
				UniformContents::Float {value: _} |
				UniformContents::Vector3 {value: _} |
				UniformContents::Matrix4 {value: _} => {
					offset += get_byte(contents);
				},
				_ => {},
			};
		}
	}

	fn build_layout(
		device: &wgpu::Device,
		pools: &ResourcePools,
		material: &Material
	) -> wgpu::BindGroupLayout {
		let mut entries = Vec::new();
		let mut buffer_size = 0;
		let mut max_align = 0;

		// binding 0 : Object (model-view matrix, normal matrix)
		// binding 1 : Camera (projection matrix)
		// binding 2 : Uniform buffers
		// binding 3- : Textures
		// binding n- : Samplers

		for contents in material.borrow_contents(
			pools.borrow::<Box<dyn MaterialNode>>(),
		).iter() {
			match contents {
				UniformContents::Float {..} |
				UniformContents::Matrix4 {..} |
				UniformContents::Vector3 {..} => {
					let align = get_align(contents);

					max_align = if align > max_align {
						align
					} else {
						max_align
					};

					buffer_size += (align - (buffer_size % align)) % align;
					buffer_size += get_byte(contents);
				},
				UniformContents::Texture {..} => {},
			};
		}

		for _texture in material.borrow_textures(
			pools.borrow::<Box<dyn MaterialNode>>(),
		).iter() {
			entries.push(wgpu::BindGroupLayoutEntry {
				binding: entries.len() as u32 + 3,
				count: None,
				ty: wgpu::BindingType::Texture {
					multisampled: false,
					sample_type: wgpu::TextureSampleType::Float {
						filterable: true,
					},
					view_dimension: wgpu::TextureViewDimension::D2,
				},
				// @TODO: Fix me
				visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
			});
		}

		// @TODO: Fix me. Loop twice is inefficient
		for _sampler in material.borrow_samplers(
			pools.borrow::<Box<dyn MaterialNode>>(),
		).iter() {
			entries.push(wgpu::BindGroupLayoutEntry {
				binding: entries.len() as u32 + 3,
				count: None,
				// @TODO: Fix me if needed
				ty: wgpu::BindingType::Sampler {
					filtering: true,
					comparison: false,
				},
				// @TODO: Fix me
				visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
			});
		}

		buffer_size += (max_align - (buffer_size % max_align)) % max_align;

		entries.push(wgpu::BindGroupLayoutEntry {
			binding: 0,
			count: None,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Uniform,
				has_dynamic_offset: false,
				min_binding_size: wgpu::BufferSize::new((16 + 12) * 4),
			},
			visibility: wgpu::ShaderStages::VERTEX,
		});

		entries.push(wgpu::BindGroupLayoutEntry {
			binding: 1,
			count: None,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Uniform,
				has_dynamic_offset: false,
				min_binding_size: wgpu::BufferSize::new(16 * 4),
			},
			visibility: wgpu::ShaderStages::VERTEX,
		});

		entries.push(wgpu::BindGroupLayoutEntry {
			binding: 2,
			count: None,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Uniform,
				has_dynamic_offset: false,
				min_binding_size: wgpu::BufferSize::new(buffer_size),
			},
			// @TODO: Fix me
			visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
		});

		device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &entries,
			label: None,
		})
	}

	fn build_group(
		device: &wgpu::Device,
		layout: &wgpu::BindGroupLayout,
		buffers: &Vec<wgpu::Buffer>,
		textures: &Vec<&wgpu::Texture>,
		samplers: &Vec<&wgpu::Sampler>,
	) -> wgpu::BindGroup {
		let mut entries = Vec::new();

		for buffer in buffers.iter() {
			entries.push(wgpu::BindGroupEntry {
				binding: entries.len() as u32,
				resource: buffer.as_entire_binding(),
			});
		}

		let mut texture_views = Vec::new();
		for texture in textures.iter() {
			texture_views.push(texture.create_view(&wgpu::TextureViewDescriptor::default()));
		}

		for texture_view in texture_views.iter() {
			entries.push(wgpu::BindGroupEntry {
				binding: entries.len() as u32,
				resource: wgpu::BindingResource::TextureView(&texture_view),
			});
		}

		for sampler in samplers.iter() {
			entries.push(wgpu::BindGroupEntry {
				binding: entries.len() as u32,
				resource: wgpu::BindingResource::Sampler(sampler),
			});
		}

		device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &layout,
			entries: &entries,
			label: None,
		})
	}

	fn build_buffers(
		device: &wgpu::Device,
		pools: &ResourcePools,
		material: &Material,
	) -> Vec<wgpu::Buffer> {
		let mut buffers = Vec::new();

		// binding 0 : Object (model-view matrix, normal matrix)
		// binding 1 : Camera (projection matrix)
		// binding 2 : Uniform buffers

		buffers.push(create_buffer(device, (16 + 9) * 4));
		buffers.push(create_buffer(device, 16 * 4));

		let mut buffer_size = 0;
		for contents in material.borrow_contents(
			pools.borrow::<Box<dyn MaterialNode>>(),
		).iter() {
			match contents {
				UniformContents::Float {value: _} |
				UniformContents::Matrix4 {value: _} |
				UniformContents::Vector3 {value: _} => {
					let align = get_align(contents);
					buffer_size += (align - (buffer_size % align)) % align;
					buffer_size += get_byte(contents);
				},
				_ => {},
			};
		}

		buffers.push(create_buffer(device, buffer_size as usize));
		buffers
	}
}

pub struct WGPUBindings {
	groups: HashMap<ResourceId<Node>, WGPUBinding>
}

// @TODO: Implement correctly
impl WGPUBindings {
	pub fn new() -> Self {
		WGPUBindings {
			groups: HashMap::new()
		}
	}

	pub fn borrow(&self, node: &ResourceId<Node>) -> Option<&WGPUBinding> {
		self.groups.get(node)
	}

	pub fn update(&mut self,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		wgpu_textures: &WGPUTextures,
		wgpu_samplers: &WGPUSamplers,
		pools: &ResourcePools,
		node_rid: &ResourceId<Node>,
		camera: &PerspectiveCamera,
		camera_node: &Node,
		material: &Material,
	) {
		if !self.groups.contains_key(node_rid) {
			self.groups.insert(*node_rid, WGPUBinding::new(
				device,
				wgpu_textures,
				wgpu_samplers,
				pools,
				material
			));
		}

		if let Some(node) = pools.borrow::<Node>().borrow(node_rid) {
			let binding = self.groups.get(node_rid).unwrap();
			binding.update(queue, pools, node, camera, camera_node, material);
		}
	}
}

fn create_buffer(device: &wgpu::Device, size_in_byte: usize) -> wgpu::Buffer {
	use wgpu::util::DeviceExt;
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: None,
		contents: bytemuck::cast_slice(&vec![0.0; size_in_byte / 4]),
		usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
	})
}

fn get_byte(contents: &UniformContents) -> u64 {
	match contents {
		UniformContents::Float {value: _} => 4,
		UniformContents::Matrix4 {value: _} => 64,
		UniformContents::Vector3 {value: _} => 12,
		_ => 0,
	}
}

fn get_align(contents: &UniformContents) -> u64 {
	match contents {
		UniformContents::Float {value: _} => 4,
		UniformContents::Matrix4 {value: _} => 64,
		UniformContents::Vector3 {value: _} => 16,
		_ => 0,
	}
}
