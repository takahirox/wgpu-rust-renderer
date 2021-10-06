use crate::{
	math::{
		matrix3::Matrix3,
		matrix3gpu::Matrix3GPU,
		matrix4::Matrix4,
	},
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		object::Object,
	},
};

pub struct WGPUBinding {
	buffers: Vec<wgpu::Buffer>,
	group: wgpu::BindGroup,
	layout: wgpu::BindGroupLayout,
}

impl WGPUBinding {
	fn new(
		layout: wgpu::BindGroupLayout,
		group: wgpu::BindGroup,
		buffers: Vec<wgpu::Buffer>,
	) -> Self {
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
}

pub struct WGPUBindings {
	groups: Vec<WGPUBinding>
}

// @TODO: Implement correctly
impl WGPUBindings {
	pub fn new() -> Self {
		WGPUBindings {
			groups: Vec::new()
		}
	}

	pub fn borrow(&self) -> &WGPUBinding {
		&self.groups.last().unwrap()
	}

	pub fn update(&mut self,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		object: &Object,
		camera: &PerspectiveCamera,
		camera_object: &Object,
		mesh: &Mesh,
		texture: &wgpu::Texture,
	) {
		if self.groups.len() == 0 {
			let mut buffers = Vec::new();
			buffers.push(create_buffer(device, (16 + 9) * 4)); // model-view matrix, normal matrix
			buffers.push(create_buffer(device, 16 * 4)); // projection matrix
			buffers.push(create_buffer(device, 3 * 4)); // color
			let layout = create_layout(device);
			let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
			let group = create_group(device, &layout, &buffers, &texture_view);
			self.groups.push(WGPUBinding::new(layout, group, buffers));
		}

		// @TODO: Is calculating them here inefficient?
		let mut model_view_matrix = Matrix4::create();
		let mut camera_matrix_inverse = Matrix4::create();
		let mut normal_matrix = Matrix3::create();
		let mut normal_matrix_gpu = Matrix3GPU::create();
		Matrix4::copy(&mut camera_matrix_inverse, camera_object.borrow_matrix());
		Matrix4::invert(&mut camera_matrix_inverse);
		Matrix4::multiply(&mut model_view_matrix, &camera_matrix_inverse, object.borrow_matrix());
		Matrix3::make_normal_from_matrix4(&mut normal_matrix, &model_view_matrix);
		Matrix3GPU::copy_from_matrix3(&mut normal_matrix_gpu, &normal_matrix);

		// @TODO: Should we calculate projection matrix * model-view matrix in CPU?
		let binding = self.groups.last().unwrap();
		queue.write_buffer(binding.borrow_buffer(0), 0, bytemuck::cast_slice(&model_view_matrix));
		queue.write_buffer(binding.borrow_buffer(0), 64, bytemuck::cast_slice(&normal_matrix_gpu));
		queue.write_buffer(binding.borrow_buffer(1), 0, bytemuck::cast_slice(camera.borrow_projection_matrix()));
		queue.write_buffer(binding.borrow_buffer(2), 0, bytemuck::cast_slice(mesh.borrow_material().borrow_color()));
	}
}

fn create_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
	// @TODO: Should be programmable
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		entries: &[
			// model-view matrix, normal matrix
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				count: None,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					// mat3x3 requires 48 bytes, not 36 bytes
					min_binding_size: wgpu::BufferSize::new(64 + 48),
				},
				visibility: wgpu::ShaderStages::VERTEX,
			},
			// projection matrix
			wgpu::BindGroupLayoutEntry {
				binding: 1,
				count: None,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(64),
				},
				visibility: wgpu::ShaderStages::VERTEX,
			},
			// color
			wgpu::BindGroupLayoutEntry {
				binding: 2,
				count: None,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					// color is 12 bytes but it seems to require 16-byte boundary
					min_binding_size: wgpu::BufferSize::new(16),
				},
				visibility: wgpu::ShaderStages::FRAGMENT,
			},
			// color texture
			wgpu::BindGroupLayoutEntry {
				binding: 3,
				count: None,
				ty: wgpu::BindingType::Texture {
					multisampled: false,
					sample_type: wgpu::TextureSampleType::Float {
						filterable: false,
					},
					view_dimension: wgpu::TextureViewDimension::D2,
				},
				visibility: wgpu::ShaderStages::FRAGMENT,
			},
		],
		label: None,
	})
}

fn create_group(
	device: &wgpu::Device,
	layout: &wgpu::BindGroupLayout,
	buffers: &Vec<wgpu::Buffer>,
	texture_view: &wgpu::TextureView,
) -> wgpu::BindGroup {
	// @TODO: Programmable
	device.create_bind_group(&wgpu::BindGroupDescriptor {
		layout: &layout,
		entries: &[
			// model-view matrix, normal matrix
			wgpu::BindGroupEntry {
				binding: 0,
				resource: buffers[0].as_entire_binding(),
			},
			// projection matrix
			wgpu::BindGroupEntry {
				binding: 1,
				resource: buffers[1].as_entire_binding(),
			},
			// color
			wgpu::BindGroupEntry {
				binding: 2,
				resource: buffers[2].as_entire_binding(),
			},
			wgpu::BindGroupEntry {
				binding: 3,
				resource: wgpu::BindingResource::TextureView(&texture_view),
			},
		],
		label: None,
	})
}

fn create_buffer(device: &wgpu::Device, size_in_byte: usize) -> wgpu::Buffer {
	use wgpu::util::DeviceExt;
	device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
		label: None,
		contents: bytemuck::cast_slice(&vec![0.0; size_in_byte / 4]),
		usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
	})
}
