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
		geometry::Geometry,
		mesh::Mesh,
		scene::Scene,
	}
};

fn create_scene() -> Scene {
	let mut scene = Scene::new();
	let mut attribute_manager = AttributeManager::new();

	let mut geometry = Geometry::new();

	geometry.set_attribute("position", attribute_manager.create(
		[
			-0.5, -0.5, 0.0,
			0.5, -0.5, 0.0,
			-0.5, 0.5, 0.0,
			0.5, -0.5, 0.0,
			0.5, 0.5, 0.0,
			-0.5, 0.5, 0.0,
		].to_vec(),
		3,
	));

	geometry.set_attribute("normal", attribute_manager.create(
		[
			0.0, 0.0, 0.0,
			0.0, 0.0, 0.0,
			0.0, 0.0, 0.0,
			0.0, 0.0, 0.0,
			0.0, 0.0, 0.0,
			0.0, 0.0, 0.0,
		].to_vec(),
		3,
	));

	let mut material = Material::new();
	Color::set(material.borrow_color_mut(), 0.0, 1.0, 0.0);

	let mesh = Mesh::new(geometry, material);
	let id = scene.create_object();
	scene.add_mesh(id, mesh);

	scene
}

fn animate(scene: &mut Scene) {
	let object = scene.borrow_object_mut(0).unwrap();
	object.borrow_rotation_mut()[2] += 0.01;
	object.update_matrix();
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

	let mut scene = create_scene();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				renderer.set_size(size.width as f64, size.height as f64);
			},
			Event::RedrawEventsCleared => {
                window.request_redraw();
            },
			Event::RedrawRequested(_) => {
				animate(&mut scene);
				renderer.render(&scene);
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
