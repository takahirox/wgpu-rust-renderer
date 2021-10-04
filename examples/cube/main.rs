use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};
use wgpu_rust_renderer::{
	material::material::Material,
	math::{
		color::Color,
		vector3::Vector3,
	},
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
			// top-left-front
			-0.5, -0.5, -0.5,
			// top-right-front
			0.5, -0.5, -0.5,
			// bottom-left-front
			-0.5, 0.5, -0.5,
			// bottom-right-front
			0.5, 0.5, -0.5,
			// top-left-back
			-0.5, -0.5, 0.5,
			// top-right-back
			0.5, -0.5, 0.5,
			// bottom-left-back
			-0.5, 0.5, 0.5,
			// bottom-right-back
			0.5, 0.5, 0.5,
		].to_vec(),
		3,
	));

	geometry.set_attribute("normal", attribute_manager.create(
		[
			// top-left-front
			-0.5, -0.5, -0.5,
			// top-right-front
			0.5, -0.5, -0.5,
			// bottom-left-front
			-0.5, 0.5, -0.5,
			// bottom-right-front
			0.5, 0.5, -0.5,
			// top-left-back
			-0.5, -0.5, 0.5,
			// top-right-back
			0.5, -0.5, 0.5,
			// bottom-left-back
			-0.5, 0.5, 0.5,
			// bottom-right-back
			0.5, 0.5, 0.5,
		].to_vec(),
		3,
	));

	geometry.set_index(index_manager.create(
		[
			// front-face
			0, 1, 2,
			1, 3, 2,
			// left-face
			4, 0, 6,
			0, 2, 6,
			// right-face
			1, 5, 3,
			5, 7, 3,
			// back-face
			5, 4, 7,
			4, 6, 7,
			// top-face
			4, 5, 0,
			5, 1, 0,
			// bottom-face
			2, 3, 6,
			3, 7, 6,
		].to_vec()
	));

	let mut material = Material::new();
	Color::set(material.borrow_color_mut(), 0.5, 0.5, 1.0);

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
	Vector3::set(
		scene
			.borrow_object_mut(id)
			.unwrap()
			.borrow_position_mut(),
		0.0, 0.0, 3.0,
	);

	scene
}

fn resize(renderer: &mut WGPURenderer, scene: &mut Scene, width: u32, height: u32) {
	scene.borrow_active_camera_mut().unwrap().set_aspect(width as f32 / height as f32);
	renderer.set_size(width as f64, height as f64);
	render(renderer, scene);
}

fn animate(scene: &mut Scene) {
	let object = scene.borrow_object_mut(0).unwrap();
	Vector3::add(
		object.borrow_rotation_mut(),
		&[0.001, 0.01, 0.003],
	);
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