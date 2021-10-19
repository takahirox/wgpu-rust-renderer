#[path = "../../utils/window.rs"]
mod window;

use wasm_bindgen::prelude::*;
use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
};

use window::{
	create_window,
	get_window_device_pixel_ratio,
	get_window_inner_size,
};

use wgpu_rust_renderer::{
	math::color::Color,
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
	utils::{
		geometry_helper::GeometryHelper,
		material_helper::MaterialHelper,
	},
	web::wgpu_web_renderer::WGPUWebRenderer,
};

fn create_scene(
	pools: &mut ResourcePools
) -> (ResourceId<Scene>, ResourceId<PerspectiveCamera>) {
	let mut scene = Scene::new();

	let geometry = GeometryHelper::create_triangle(
		pools,
		1.0,
		1.0,
	);

	let material = MaterialHelper::create_basic_material(
		pools,
		Color::set(&mut Color::create(), 1.0, 0.0, 0.0),
	);

	let mesh = pools.borrow_mut::<Mesh>().add(Mesh::new(geometry, material));
	let node = pools.borrow_mut::<Node>().add(Node::new());
	scene.add_node(&node);
	scene.assign(&node, &mesh);

	let window_size = get_window_inner_size();
	let camera = pools.borrow_mut::<PerspectiveCamera>().add(
		PerspectiveCamera::new(
			60.0_f32.to_radians(),
			(window_size.0 / window_size.1) as f32,
			0.1,
			1000.0,
		),
	);

	let mut node = Node::new();
	node.borrow_position_mut()[2] = 1.0;

	let node = pools.borrow_mut::<Node>().add(node);
	scene.add_node(&node);
	scene.assign(&node, &camera);

	(pools.borrow_mut::<Scene>().add(scene), camera)
}

fn resize(
	renderer: &mut WGPUWebRenderer,
	pools: &mut ResourcePools,
	camera: &ResourceId<PerspectiveCamera>,
	width: u32,
	height: u32,
) {
	pools
		.borrow_mut::<PerspectiveCamera>()
		.borrow_mut(camera)
		.unwrap()
		.set_aspect(width as f32 / height as f32);
	renderer.set_size(width as f64, height as f64);
}

fn update(
	pools: &mut ResourcePools,
	scene: &ResourceId<Scene>,
) {
	pools.borrow::<Scene>()
		.borrow(scene)
		.unwrap()
		.update_matrices(pools);
}

fn render(
	renderer: &mut WGPUWebRenderer,
	pools: &ResourcePools,
	scene: &ResourceId<Scene>,
	camera: &ResourceId<PerspectiveCamera>,
) {
	renderer.render(pools, scene, camera);
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

	let mut pools = ResourcePools::new();
	let (scene, camera) = create_scene(&mut pools);

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				resize(&mut renderer, &mut pools, &camera, size.width, size.height);
				update(&mut pools, &scene);
				render(&mut renderer, &mut pools, &scene, &camera);
			},
			Event::RedrawRequested(_) => {
				update(&mut pools, &scene);
				render(&mut renderer, &mut pools, &scene, &camera);
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
