use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};
use wgpu_rust_renderer::{
	math::color::Color,
	renderer::wgpu_renderer::WGPURenderer,
	scene::{
		camera::PerspectiveCamera,
		mesh::Mesh,
		scene::Scene,
	},
	utils::{
		geometry_helper::GeometryHelper,
		material_helper::MaterialHelper,
	}
};

fn create_scene(window: &Window) -> Scene {
	let mut scene = Scene::new();

	let geometry = GeometryHelper::create_triangle(
		1.0,
		1.0,
	);

	let material = MaterialHelper::create_basic_material(
		Color::set(&mut Color::create(), 1.0, 0.0, 0.0),
	);

	let mesh = Mesh::new(geometry, material);
	let id = scene.create_node();
	scene.add_mesh(id, mesh);

	let window_size = window.inner_size();
	let camera = PerspectiveCamera::new(
		60.0_f32.to_radians(),
		window_size.width as f32 / window_size.height as f32,
		0.1,
		1000.0,
	);
	let id = scene.create_node();
	scene.add_camera(id, camera);
	scene.set_active_camera_id(id);

	scene
		.borrow_node_mut(id)
		.unwrap()
		.borrow_position_mut()[2] = 1.0;

	scene
}

fn resize(renderer: &mut WGPURenderer, scene: &mut Scene, width: u32, height: u32) {
	scene.borrow_active_camera_mut().unwrap().set_aspect(width as f32 / height as f32);
	renderer.set_size(width as f64, height as f64);
	render(renderer, scene);
}

fn render(renderer: &mut WGPURenderer, scene: &mut Scene) {
	scene.update_matrices();
	renderer.render(scene);
}

#[tokio::main]
async fn main() {
	let event_loop = EventLoop::new();
	let window = Window::new(&event_loop).unwrap();

	let window_size = window.inner_size();
	let pixel_ratio = window.scale_factor();

	let mut renderer = WGPURenderer::new(&window).await;
	renderer.set_size(window_size.width as f64, window_size.height as f64);
	renderer.set_pixel_ratio(pixel_ratio);

	let mut scene = create_scene(&window);

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				resize(&mut renderer, &mut scene, size.width, size.height);
			},
			Event::RedrawRequested(_) => {
				render(&mut renderer, &mut scene);
			},
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				*control_flow = ControlFlow::Exit;
			},
			_ => {}
		}
	});
}
