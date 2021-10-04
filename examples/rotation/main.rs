use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};
use wgpu_rust_renderer::{
	material::material::Material,
	math::color::Color,
	renderer::wgpu_renderer::WGPURenderer,
	scene::{
		attribute::AttributeManager,
		camera::PerspectiveCamera,
		geometry::Geometry,
		index::IndexManager,
		mesh::Mesh,
		scene::Scene,
	}
};

fn create_scene(window: &Window) -> Scene {
	let mut scene = Scene::new();
	let mut attribute_manager = AttributeManager::new();
	let mut index_manager = IndexManager::new();

	let mut geometry = Geometry::new();

	geometry.set_attribute("position", attribute_manager.create(
		[
			// top-left
			-0.5, -0.5, 0.0,
			// top-right
			0.5, -0.5, 0.0,
			// bottom-left
			-0.5, 0.5, 0.0,
			// bottom-right
			0.5, 0.5, 0.0,
		].to_vec(),
		3,
	));

	geometry.set_attribute("normal", attribute_manager.create(
		[
			// top-left
			0.0, 0.0, 1.0,
			// top-right
			0.0, 0.0, 1.0,
			// bottom-left
			0.0, 0.0, 1.0,
			// bottom-right
			0.0, 0.0, 1.0,
		].to_vec(),
		3,
	));

	geometry.set_index(index_manager.create(
		[
			0, 1, 2,
			1, 3, 2,
		].to_vec()
	));

	let mut material = Material::new();
	Color::set(material.borrow_color_mut(), 0.0, 1.0, 0.0);

	let mesh = Mesh::new(geometry, material);
	let id = scene.create_object();
	scene.add_mesh(id, mesh);

	let window_size = window.inner_size();
	let camera = PerspectiveCamera::new(
		60.0_f32.to_radians(),
		window_size.width as f32 / window_size.height as f32,
		0.1,
		1000.0,
	);
	let id = scene.create_object();
	scene.add_camera(id, camera);
	scene.set_active_camera_id(id);

	scene
		.borrow_object_mut(id)
		.unwrap()
		.borrow_position_mut()[2] = 2.0;

	scene
}

fn resize(renderer: &mut WGPURenderer, scene: &mut Scene, width: u32, height: u32) {
	scene.borrow_active_camera_mut().unwrap().set_aspect(width as f32 / height as f32);
	renderer.set_size(width as f64, height as f64);
	render(renderer, scene);
}

fn animate(scene: &mut Scene) {
	let object = scene.borrow_object_mut(0).unwrap();
	object.borrow_rotation_mut()[2] += 0.01;
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
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				resize(&mut renderer, &mut scene, size.width, size.height);
			},
			Event::RedrawEventsCleared => {
                window.request_redraw();
            },
			Event::RedrawRequested(_) => {
				animate(&mut scene);
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
