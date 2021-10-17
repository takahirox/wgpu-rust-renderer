use std::borrow::Cow;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
	material::material::Material,
	scene::node::Node,
};

pub struct WGPURenderPipeline {
	pipeline: wgpu::RenderPipeline
}

impl WGPURenderPipeline {
	fn new(
		device: &wgpu::Device,
		adapter: &wgpu::Adapter,
		surface: &wgpu::Surface,
		bind_group_layout: &wgpu::BindGroupLayout,
		shader_code: &str,
	) -> Self {
		// For debug
		//println!("{}", shader_code);

		let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_code)),
		});

		let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[bind_group_layout],
			push_constant_ranges: &[],
		});

		let swapchain_format = surface.get_preferred_format(&adapter).unwrap();

		// @TODO: Programmable
		let vertex_buffers = [
			// position
			wgpu::VertexBufferLayout {
				array_stride: 3 * 4,
				step_mode: wgpu::VertexStepMode::Vertex,
				attributes: &[
					wgpu::VertexAttribute {
						format: wgpu::VertexFormat::Float32x3,
						offset: 0,
						shader_location: 0,
					}
				],
			},
			// normal
			wgpu::VertexBufferLayout {
				array_stride: 3 * 4,
				step_mode: wgpu::VertexStepMode::Vertex,
				attributes: &[
					wgpu::VertexAttribute {
						format: wgpu::VertexFormat::Float32x3,
						offset: 0,
						shader_location: 1,
					},
				],
			},
			// uv
			wgpu::VertexBufferLayout {
				array_stride: 2 * 4,
				step_mode: wgpu::VertexStepMode::Vertex,
				attributes: &[
					wgpu::VertexAttribute {
						format: wgpu::VertexFormat::Float32x2,
						offset: 0,
						shader_location: 2,
					},
				],
			},
		];

		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: None,
			layout: Some(&layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &vertex_buffers,
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[swapchain_format.into()],
			}),
			// Backface culling
			// @TODO: Should be configurable 
			primitive: wgpu::PrimitiveState {
				cull_mode: Some(wgpu::Face::Back),
				front_face: wgpu::FrontFace::Ccw,
				..Default::default()
			},
			depth_stencil: Some(wgpu::DepthStencilState {
				bias: wgpu::DepthBiasState::default(),
				depth_compare: wgpu::CompareFunction::LessEqual,
				depth_write_enabled: true,
				format: wgpu::TextureFormat::Depth24PlusStencil8,
				stencil: wgpu::StencilState::default(),
			}),
			multisample: wgpu::MultisampleState::default(),
		});

		WGPURenderPipeline {
			pipeline: pipeline
		}
	}
}

pub struct WGPURenderPipelines {
	pipelines: HashMap::<Uuid, WGPURenderPipeline>
}

impl WGPURenderPipelines {
	pub fn new() -> Self {
		WGPURenderPipelines {
			pipelines: HashMap::new()
		}
	}

	pub fn borrow(&self, node: &Node) -> &wgpu::RenderPipeline {
		&self.pipelines.get(&node.get_id()).unwrap().pipeline
	}

	pub fn update(
		&mut self,
		device: &wgpu::Device,
		adapter: &wgpu::Adapter,
		surface: &wgpu::Surface,
		node: &Node,
		material: &Material,
		bind_group_layout: &wgpu::BindGroupLayout,
	) {
		if !self.pipelines.contains_key(&node.get_id()) {
			self.pipelines.insert(
				node.get_id(),
				WGPURenderPipeline::new(
					device,
					adapter,
					surface,
					bind_group_layout,
					&material.build_shader_code(),
				)
			);
		}
	}
}
