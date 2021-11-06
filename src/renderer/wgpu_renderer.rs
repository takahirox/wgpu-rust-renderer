use winit::window::Window;

use crate::{
	geometry::{
		attribute::Attribute,
		geometry::Geometry,
		index::Index,
	},
	material::material::Material,
	renderer::{
		wgpu_attributes::WGPUAttributes,
		wgpu_bindings::WGPUBindings,
		wgpu_indices::WGPUIndices,
		wgpu_render_pipeline::WGPURenderPipelines,
		wgpu_samplers::WGPUSamplers,
		wgpu_textures::WGPUTextures,
	},
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		node::Node,
		scene::Scene,
	},
};

pub struct WGPURendererOptions {
	pub sample_count: u32,
}

impl Default for WGPURendererOptions {
	fn default() -> Self {
		WGPURendererOptions {
			sample_count: 4,
		}
	}
}

pub struct WGPURenderer {
	attributes: WGPUAttributes,
	bindings: WGPUBindings,
	color_buffer: wgpu::Texture,
	device: wgpu::Device,
	depth_buffer: wgpu::Texture,
	height: f64,
	indices: WGPUIndices,
	pixel_ratio: f64,
	queue: wgpu::Queue,
	render_pipelines: WGPURenderPipelines,
	sample_count: u32,
	samplers: WGPUSamplers,
	surface: wgpu::Surface,
	surface_configuration: wgpu::SurfaceConfiguration,
	textures: WGPUTextures,
	width: f64,
}

impl WGPURenderer {
	pub async fn new(window: &Window, options: WGPURendererOptions) -> Self {
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

		let surface_configuration = wgpu::SurfaceConfiguration {
			// @TODO: Color management
			format: wgpu::TextureFormat::Bgra8Unorm,
			height: (height * pixel_ratio) as u32,
			present_mode: wgpu::PresentMode::Mailbox,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			width: (width * pixel_ratio) as u32,
		};

		surface.configure(&device, &surface_configuration);

		WGPURenderer {
			attributes: WGPUAttributes::new(),
			bindings: WGPUBindings::new(),
			color_buffer: create_color_buffer(
				&device,
				width,
				height,
				pixel_ratio,
				options.sample_count,
			),
			depth_buffer: create_depth_buffer(
				&device,
				width,
				height,
				pixel_ratio,
				options.sample_count,
			),
			device: device,
			height: height,
			indices: WGPUIndices::new(),
			pixel_ratio: pixel_ratio,
			queue: queue,
			render_pipelines: WGPURenderPipelines::new(),
			sample_count: options.sample_count,
			samplers: WGPUSamplers::new(),
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
		self.recreate_color_buffer();
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

	fn update(
		&mut self,
		pools: &ResourcePools,
		scene_rid: &ResourceId<Scene>,
		camera_rid: &ResourceId<PerspectiveCamera>,
	) {
		let geometry_pool = pools.borrow::<Geometry>();
		let mesh_pool = pools.borrow::<Mesh>();
		let material_pool = pools.borrow::<Material>();
		let node_pool = pools.borrow::<Node>();

		// @TODO: Error handling

		let scene = match pools.borrow::<Scene>().borrow(scene_rid) {
			Some(scene) => scene,
			None => return,
		};

		let camera = match pools.borrow::<PerspectiveCamera>().borrow(camera_rid) {
			Some(camera) => camera,
			None => return,
		};

		let camera_node = match scene.borrow_assigned_from::<PerspectiveCamera>(camera_rid) {
			Some(rid) => match node_pool.borrow(rid) {
				Some(node) => node,
				None => return,
			},
			None => return,
		};

		for node_rid in scene.collect_nodes(pools).iter() {
			let mesh = match scene.borrow_assigned_to::<Mesh>(node_rid) {
				Some(rid) => match mesh_pool.borrow(rid) {
					Some(mesh) => mesh,
					None => continue,
				},
				None => continue,
			};

			let geometry = match geometry_pool.borrow(mesh.borrow_geometry()) {
				Some(geometry) => geometry,
				None => continue,
			};

			let material = match material_pool.borrow(mesh.borrow_material()) {
				Some(material) => material,
				None => continue,
			};

			// @TODO: Implement correctly
			if let Some(rid) = geometry.borrow_attribute("position") {
				self.attributes.update(&self.device, pools, rid);
			}
			if let Some(rid) = geometry.borrow_attribute("normal") {
				self.attributes.update(&self.device, pools, rid);
			}
			if let Some(rid) = geometry.borrow_attribute("uv") {
				self.attributes.update(&self.device, pools, rid);
			}

			if let Some(rid) = geometry.borrow_index() {
				self.indices.update(&self.device, pools, rid);
			}

			self.textures.update_from_material(
				&self.device,
				&self.queue,
				pools,
				material,
			);

			self.samplers.update_from_material(
				&self.device,
				pools,
				material,
			);

			self.bindings.update(
				&self.device,
				&self.queue,
				&self.textures,
				&self.samplers,
				pools,
				node_rid,
				camera,
				camera_node,
				material,
			);

			self.render_pipelines.update(
				&self.device,
				pools,
				node_rid,
				material,
				&self.bindings.borrow(node_rid).unwrap().borrow_layout(),
				self.sample_count,
			);
		}
	}

	fn render_internal(
		&self,
		pools: &ResourcePools,
		scene_rid: &ResourceId<Scene>,
	) {
		let attribute_pool = pools.borrow::<Attribute>();
		let geometry_pool = pools.borrow::<Geometry>();
		let index_pool = pools.borrow::<Index>();
		let mesh_pool = pools.borrow::<Mesh>();

		// @TODO: Error handling

		let scene = match pools.borrow::<Scene>().borrow(scene_rid) {
			Some(scene) => scene,
			None => return,
		};

		let frame = self.surface
			.get_current_texture()
			.expect("Failed to acquire next swap chain texture");

		let view = frame
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let color_view = &self.color_buffer.create_view(&wgpu::TextureViewDescriptor::default());
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
					resolve_target: match self.sample_count {
						1 => None,
						_ => Some(&view),
					},
					view: match self.sample_count {
						1 => &view,
						_ => &color_view,
					},
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

			for node_rid in scene.collect_nodes(pools).iter() {
				let mesh = match scene.borrow_assigned_to::<Mesh>(node_rid) {
					Some(rid) => match mesh_pool.borrow(rid) {
						Some(mesh) => mesh,
						None => continue,
					},
					None => continue,
				};

				let geometry = match geometry_pool.borrow(mesh.borrow_geometry()) {
					Some(geometry) => geometry,
					None => continue,
				};

				if let Some(pipeline) = self.render_pipelines.borrow(node_rid) {
					pass.set_pipeline(&pipeline);
				} else {
					continue;
				}

				// @TODO: Should be programmable
				if let Some(rid) = geometry.borrow_attribute("position") {
					if let Some(buffer) = self.attributes.borrow(rid) {
						pass.set_vertex_buffer(0, buffer.slice(..));
					}
				}
				if let Some(rid) = geometry.borrow_attribute("normal") {
					if let Some(buffer) = self.attributes.borrow(rid) {
						pass.set_vertex_buffer(1, buffer.slice(..));
					}
				}
				if let Some(rid) = geometry.borrow_attribute("uv") {
					if let Some(buffer) = self.attributes.borrow(rid) {
						pass.set_vertex_buffer(2, buffer.slice(..));
					}
				}

				let binding = self.bindings.borrow(node_rid).unwrap();
				pass.set_bind_group(0, &binding.borrow_group(), &[]);

				if let Some(rid) = geometry.borrow_index() {
					if let Some(indices) = index_pool.borrow(rid) {
						if let Some(buffer) = self.indices.borrow(rid) {
							pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint16);
							pass.draw_indexed(0..indices.get_count(), 0, 0..1);
						}
					}
				} else {
					if let Some(rid) = geometry.borrow_attribute("position") {
						if let Some(positions) = attribute_pool.borrow(rid) {
							pass.draw(0..positions.get_count(), 0..1);
						}
					}
				}
			}
		}

		self.queue.submit(Some(encoder.finish()));
		frame.present()
	}

	pub fn render(
		&mut self,
		pools: &ResourcePools,
		scene_rid: &ResourceId<Scene>,
		camera_rid: &ResourceId<PerspectiveCamera>,
	) {
		self.update(pools, scene_rid, camera_rid);
		self.render_internal(pools, scene_rid);
	}

	fn update_surface_configuration(&mut self) {
		self.surface_configuration.width = (self.width * self.pixel_ratio) as u32;
		self.surface_configuration.height = (self.height * self.pixel_ratio) as u32;
		self.surface.configure(&self.device, &self.surface_configuration);
	}

	fn recreate_color_buffer(&mut self) {
		self.color_buffer.destroy();
		self.color_buffer = create_color_buffer(
			&self.device,
			self.width,
			self.height,
			self.pixel_ratio,
			self.sample_count,
		);
	}

	fn recreate_depth_buffer(&mut self) {
		self.depth_buffer.destroy();
		self.depth_buffer = create_depth_buffer(
			&self.device,
			self.width,
			self.height,
			self.pixel_ratio,
			self.sample_count,
		);
	}
}

fn create_color_buffer(
	device: &wgpu::Device,
	width: f64,
	height: f64,
	pixel_ratio: f64,
	sample_count: u32,
) -> wgpu::Texture {
	device.create_texture(&wgpu::TextureDescriptor {
		label: None,
		size: wgpu::Extent3d {
			width: (width * pixel_ratio) as u32,
			height: (height * pixel_ratio) as u32,
			depth_or_array_layers: 1,
		},
		mip_level_count: 1,
		sample_count: sample_count,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Bgra8Unorm,
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
	})
}

fn create_depth_buffer(
	device: &wgpu::Device,
	width: f64,
	height: f64,
	pixel_ratio: f64,
	sample_count: u32,
) -> wgpu::Texture {
	device.create_texture(&wgpu::TextureDescriptor {
		label: None,
		size: wgpu::Extent3d {
			width: (width * pixel_ratio) as u32,
			height: (height * pixel_ratio) as u32,
			depth_or_array_layers: 1,
		},
		mip_level_count: 1,
		sample_count: sample_count,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Depth24PlusStencil8,
		usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
	})
}
