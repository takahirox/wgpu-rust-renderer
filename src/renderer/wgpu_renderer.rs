use winit::window::Window;

use crate::renderer::{
	wgpu_attributes::WGPUAttributes,
	wgpu_bindings::WGPUBindings,
	wgpu_indices::WGPUIndices,
	wgpu_render_pipeline::WGPURenderPipelines,
	wgpu_textures::WGPUTextures,
};
use crate::scene::scene::Scene;

pub struct WGPURenderer {
	adapter: wgpu::Adapter,
	attributes: WGPUAttributes,
	bindings: WGPUBindings,
	device: wgpu::Device,
	depth_buffer: wgpu::Texture,
	height: f64,
	indices: WGPUIndices,
	pixel_ratio: f64,
	queue: wgpu::Queue,
	render_pipelines: WGPURenderPipelines,
	surface: wgpu::Surface,
	surface_configuration: wgpu::SurfaceConfiguration,
	textures: WGPUTextures,
	width: f64
}

impl WGPURenderer {
	pub async fn new(window: &Window) -> Self {
		let width = 640.0;
		let height = 480.0;
		let pixel_ratio = 1.0;

		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe { instance.create_surface(window) };
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
				power_preference: wgpu::PowerPreference::default(),
			})
			.await
			.expect("Failed to find an appropriate adapter");

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					features: wgpu::Features::empty(),
					label: None,
					limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
				},
				None,
			)
			.await
			.expect("Failed to create device");

		let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

		let surface_configuration = wgpu::SurfaceConfiguration {
			format: swapchain_format,
			height: (height * pixel_ratio) as u32,
			present_mode: wgpu::PresentMode::Mailbox,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			width: (width * pixel_ratio) as u32,
		};

		surface.configure(&device, &surface_configuration);

		WGPURenderer {
			adapter: adapter,
			attributes: WGPUAttributes::new(),
			bindings: WGPUBindings::new(),
			depth_buffer: create_depth_buffer(&device, width, height, pixel_ratio),
			device: device,
			height: height,
			indices: WGPUIndices::new(),
			pixel_ratio: pixel_ratio,
			queue: queue,
			render_pipelines: WGPURenderPipelines::new(),
			surface: surface,
			surface_configuration: surface_configuration,
			textures: WGPUTextures::new(),
			width: width
		}
	}

	pub fn set_size(&mut self, width: f64, height: f64) -> &mut Self {
		self.width = width;
		self.height = height;

		self.update_surface_configuration();
		self.recreate_depth_buffer();

		self
	}

	pub fn get_size(&self) -> (f64, f64) {
		(self.width, self.height)
	}

	pub fn set_pixel_ratio(&mut self, pixel_ratio: f64) -> &mut Self {
		self.pixel_ratio = pixel_ratio;
		self.set_size(self.width, self.height)
	}

	pub fn get_pixel_ratio(&self) -> f64 {
		self.pixel_ratio
	}

	pub fn render(&mut self, scene: &Scene) {
		if scene.borrow_active_camera().is_none() {
			return;
		}

		for i in 0..scene.get_nodes_num() {
			let node = scene.borrow_node(i).unwrap();
			if let Some(mesh) = scene.borrow_mesh(node.get_id()) {
				let geometry = mesh.borrow_geometry();

				// @TODO: Implement correctly
				if let Some(attribute) = geometry.borrow_attribute("position") {
					self.attributes.update(&self.device, attribute);
				}
				if let Some(attribute) = geometry.borrow_attribute("normal") {
					self.attributes.update(&self.device, attribute);
				}
				if let Some(attribute) = geometry.borrow_attribute("uv") {
					self.attributes.update(&self.device, attribute);
				}

				if let Some(indices) = geometry.borrow_index() {
					self.indices.update(&self.device, indices);
				}

				let material = mesh.borrow_material();

				// @TODO: Fix me
				let texture = material.borrow_texture().unwrap();
				self.textures.update(&self.device, &self.queue, texture);
				let texture_gpu = self.textures.borrow(texture).unwrap();

				self.bindings.update(
					&self.device,
					&self.queue,
					node,
					scene.borrow_active_camera().unwrap(),
					scene.borrow_node(scene.get_active_camera_id().unwrap()).unwrap(),
					mesh,
					texture_gpu,
				);

				self.render_pipelines.update(
					&self.device,
					&self.adapter,
					&self.surface,
					node,
					&self.bindings.borrow(node).unwrap().borrow_layout(),
				);

			}
		}

		let frame = self.surface
			.get_current_texture()
			.expect("Failed to acquire next swap chain texture");

		let view = frame
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let depth_view = &self.depth_buffer.create_view(&wgpu::TextureViewDescriptor::default());

		let background_color = scene.borrow_background_color();

		let mut encoder = self.device.create_command_encoder(
			&wgpu::CommandEncoderDescriptor {label: None});

		{
			let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: None,
				color_attachments: &[wgpu::RenderPassColorAttachment {
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: background_color[0] as f64,
							g: background_color[1] as f64,
							b: background_color[2] as f64,
							a: 1.0,
						}),
						store: true,
					},
					resolve_target: None,
					view: &view,
				}],
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(1.0),
						store: true,
					}),
					stencil_ops: None,
					view: &depth_view,
				}),
			});

			for i in 0..scene.get_nodes_num() {
				let node = scene.borrow_node(i).unwrap();
				if let Some(mesh) = scene.borrow_mesh(node.get_id()) {
					let pipeline = self.render_pipelines.borrow(node);

					pass.set_pipeline(&pipeline);

					let geometry = mesh.borrow_geometry();

					// @TODO: Should be programmable
					if let Some(positions) = geometry.borrow_attribute("position") {
						if let Some(buffer) = self.attributes.borrow(positions) {
							pass.set_vertex_buffer(0, buffer.slice(..));
						}
					}
					if let Some(normals) = geometry.borrow_attribute("normal") {
						if let Some(buffer) = self.attributes.borrow(normals) {
							pass.set_vertex_buffer(1, buffer.slice(..));
						}
					}
					if let Some(uvs) = geometry.borrow_attribute("uv") {
						if let Some(buffer) = self.attributes.borrow(uvs) {
							pass.set_vertex_buffer(2, buffer.slice(..));
						}
					}

					let binding = self.bindings.borrow(node).unwrap();
					pass.set_bind_group(0, &binding.borrow_group(), &[]);

					if let Some(indices) = geometry.borrow_index() {
						if let Some(buffer) = self.indices.borrow(indices) {
							pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint16);
							pass.draw_indexed(0..indices.get_count(), 0, 0..1);
						}
					} else {
						let positions = geometry.borrow_attribute("position").unwrap();
						pass.draw(0..positions.get_count(), 0..1);
					}
				}
			}
		}

		self.queue.submit(Some(encoder.finish()));
		frame.present()
	}

	fn update_surface_configuration(&mut self) {
		self.surface_configuration.width = (self.width * self.pixel_ratio) as u32;
		self.surface_configuration.height = (self.height * self.pixel_ratio) as u32;
		self.surface.configure(&self.device, &self.surface_configuration);
	}

	fn recreate_depth_buffer(&mut self) {
		self.depth_buffer.destroy();
		self.depth_buffer = create_depth_buffer(
			&self.device,
			self.width,
			self.height,
			self.pixel_ratio
		);
	}
}

fn create_depth_buffer(device: &wgpu::Device, width: f64, height: f64, pixel_ratio: f64) -> wgpu::Texture {
	device.create_texture(&wgpu::TextureDescriptor {
		label: None,
		size: wgpu::Extent3d {
			width: (width * pixel_ratio) as u32,
			height: (height * pixel_ratio) as u32,
			depth_or_array_layers: 1,
		},
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Depth24PlusStencil8,
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
	})
}
