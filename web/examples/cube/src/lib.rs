use wasm_bindgen::{
	prelude::*,
	JsCast,
};

use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};

use wgpu_rust_renderer::{
	material::material::Material,
	math::{
		color::Color,
		vector3::Vector3,
	},
	scene::{
		camera::PerspectiveCamera,
		attribute::AttributeManager,
		geometry::Geometry,
		index::IndexManager,
		mesh::Mesh,
		scene::Scene,
	},
	web::wgpu_web_renderer::WGPUWebRenderer,
};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

// Window and DOM element helpers

fn get_window_inner_size() -> (f64, f64) {
	let window = web_sys::window().unwrap();
	(
		window.inner_width().unwrap().as_f64().unwrap(),
		window.inner_height().unwrap().as_f64().unwrap()
	)
}

fn get_window_device_pixel_ratio() -> f64 {
	let window = web_sys::window().unwrap();
	window.device_pixel_ratio()
}

fn create_scene() -> Scene {
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

	let window_size = get_window_inner_size();
	let camera = PerspectiveCamera::new(
		60.0_f32.to_radians(),
		window_size.0 as f32 / window_size.1 as f32,
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

fn resize(renderer: &mut WGPUWebRenderer, scene: &mut Scene, width: u32, height: u32) {
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

fn render(renderer: &mut WGPUWebRenderer, scene: &mut Scene) {
	scene.update_matrices();
	renderer.render(scene);
}

fn create_window(event_loop: &EventLoop<()>) -> std::rc::Rc<winit::window::Window> {
	let window = winit::window::Window::new(&event_loop).unwrap();
	let window = std::rc::Rc::new(window);

	// winit::window::Window doesn't seem to detect browser's onresize event so we emulate it.
    {
		let window = window.clone();
		let closure = Closure::wrap(Box::new(move |_e: web_sys::Event| {
			let size = get_window_inner_size();
			window.set_inner_size(winit::dpi::PhysicalSize::new(
				size.0, size.1,
			));
		}) as Box<dyn FnMut(_)>);
		web_sys::window()
			.unwrap()
			.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
			.unwrap();
		closure.forget();
    }

	window
}

#[wasm_bindgen(start)]
pub async fn start() {
	std::panic::set_hook(Box::new(console_error_panic_hook::hook));
	console_log::init().expect("could not initialize logger");

	let event_loop = EventLoop::new();
	let window = create_window(&event_loop);

	use winit::platform::web::WindowExtWebSys;

	web_sys::window()
		.and_then(|win| win.document())
		.and_then(|doc| doc.body())
		.and_then(|body| {
			body.append_child(&web_sys::Element::from(window.canvas()))
				.ok()
		})
		.expect("couldn't append canvas to document body");

	let inner_size = get_window_inner_size();
	let pixel_ratio = get_window_device_pixel_ratio();

	let mut renderer = WGPUWebRenderer::new(&window, window.canvas()).await;
	renderer.set_size(inner_size.0 as f64, inner_size.1 as f64);
	renderer.set_pixel_ratio(pixel_ratio as f64);

	let mut scene = create_scene();

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
