use std::borrow::Cow;

pub struct WGPURenderPipeline {
	pipeline: wgpu::RenderPipeline
}

impl WGPURenderPipeline {
	fn new(device: &wgpu::Device, adapter: &wgpu::Adapter, surface: &wgpu::Surface) -> Self {
		let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
		});

		let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[],
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
			primitive: wgpu::PrimitiveState::default(),
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
		});

		WGPURenderPipeline {
			pipeline: pipeline
		}
	}
}

pub struct WGPURenderPipelines {
	pipelines: Vec<WGPURenderPipeline>
}

impl WGPURenderPipelines {
	pub fn new() -> Self {
		WGPURenderPipelines {
			pipelines: vec![]
		}
	}

	pub fn get(&mut self, device: &wgpu::Device, adapter: &wgpu::Adapter, surface: &wgpu::Surface) -> &wgpu::RenderPipeline {
		if self.pipelines.len() > 0 {
			return &self.pipelines.last().unwrap().pipeline;
		}

		self.pipelines.push(WGPURenderPipeline::new(device, adapter, surface));

		&self.pipelines.last().unwrap().pipeline
	}
}
