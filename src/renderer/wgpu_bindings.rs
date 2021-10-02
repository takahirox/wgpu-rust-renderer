use crate::scene::object::Object;

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
	) {
		if self.groups.len() == 0 {
			let mut buffers = Vec::new();
			buffers.push(create_buffer(device, 16 * 4)); // local matrix
			buffers.push(create_buffer(device, 3 * 4)); // color
			let layout = create_layout(device);
			let group = create_group(device, &layout, &buffers);
			self.groups.push(WGPUBinding::new(layout, group, buffers));
		}

		let binding = self.groups.last().unwrap();
		queue.write_buffer(binding.borrow_buffer(0), 0, bytemuck::cast_slice(object.borrow_matrix()));
		// @TODO: Fix me
		queue.write_buffer(binding.borrow_buffer(1), 0, bytemuck::cast_slice(&[1.0, 0.5, 0.1]));
	}
}

fn create_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
	// @TODO: Should be programmable
	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		entries: &[
			// local model matrix
			wgpu::BindGroupLayoutEntry {
				binding: 0,
				visibility: wgpu::ShaderStages::VERTEX,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					min_binding_size: wgpu::BufferSize::new(64),
				},
				count: None,
			},
			// color
			wgpu::BindGroupLayoutEntry {
				binding: 1,
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Buffer {
					ty: wgpu::BufferBindingType::Uniform,
					has_dynamic_offset: false,
					// color is 12 bytes but it seems to require eight-byte boundary?
					min_binding_size: wgpu::BufferSize::new(16),
				},
				count: None,
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

fn create_group(
	device: &wgpu::Device,
	layout: &wgpu::BindGroupLayout,
	buffers: &Vec<wgpu::Buffer>,
) -> wgpu::BindGroup {
	// @TODO: Programmable
	device.create_bind_group(&wgpu::BindGroupDescriptor {
		layout: &layout,
		entries: &[
			// local model matrix
			wgpu::BindGroupEntry {
				binding: 0,
				resource: buffers[0].as_entire_binding(),
			},
			// color
			wgpu::BindGroupEntry {
				binding: 1,
				resource: buffers[1].as_entire_binding(),
			},
		],
		label: None,
	})
}
