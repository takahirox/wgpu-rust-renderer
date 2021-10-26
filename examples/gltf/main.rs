use winit::{
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};
use wgpu_rust_renderer::{
	math::vector3::Vector3,
	renderer::wgpu_renderer::WGPURenderer,
	resource::resource::{
		ResourceId,
		ResourcePools,
	},
	scene::{
		camera::PerspectiveCamera,
		node::Node,
		scene::Scene,
	},
	utils::gltf_loader::GltfLoader,
};

fn create_scene(
	window: &Window,
	pools: &mut ResourcePools
) -> (ResourceId<Scene>, ResourceId<PerspectiveCamera>, Vec<ResourceId<Node>>) {
	let mut objects = Vec::new();
	let scene_rid = pools.borrow_mut::<Scene>().add(Scene::new());

	let nodes = GltfLoader::load_gltf(
		pools,
		&scene_rid,
		concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/examples/gltf/assets/",
		),
		"DamagedHelmet.gltf",
	);

	for node in nodes.iter() {
		pools.borrow_mut::<Scene>()
			.borrow_mut(&scene_rid)
			.unwrap()
			.add_node(node);
		objects.push(*node);
		pools.borrow_mut::<Node>()
			.borrow_mut(node)
			.unwrap()
			.borrow_rotation_mut()[0] = 90.0_f32.to_radians();
	}

	let window_size = window.inner_size();
	let camera = pools.borrow_mut::<PerspectiveCamera>().add(
		PerspectiveCamera::new(
			60.0_f32.to_radians(),
			window_size.width as f32 / window_size.height as f32,
			0.1,
			1000.0,
		),
	);

	let mut node = Node::new();
	Vector3::set(
		node.borrow_position_mut(),
		0.0, 0.0, 3.0,
	);

	let node = pools.borrow_mut::<Node>().add(node);

	{
		let scene = pools.borrow_mut::<Scene>().borrow_mut(&scene_rid).unwrap();
		scene.add_node(&node);
		scene.assign(&node, &camera);
	}

	(scene_rid, camera, objects)
}

fn resize(
	renderer: &mut WGPURenderer,
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
	objects: &Vec<ResourceId<Node>>,
) {
	{
		let node = pools.borrow_mut::<Node>().borrow_mut(&objects[0]).unwrap();
		Vector3::add(
			node.borrow_rotation_mut(),
			&[0.0, 0.0, -0.01],
		);
	}

	pools.borrow::<Scene>()
		.borrow(scene)
		.unwrap()
		.update_matrices(pools);
}

fn render(
	renderer: &mut WGPURenderer,
	pools: &ResourcePools,
	scene: &ResourceId<Scene>,
	camera: &ResourceId<PerspectiveCamera>,
) {
	renderer.render(pools, scene, camera);
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

	let mut pools = ResourcePools::new();
	let (scene, camera, objects) = create_scene(&window, &mut pools);

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Poll;
		match event {
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				resize(&mut renderer, &mut pools, &camera, size.width, size.height);
				update(&mut pools, &scene, &objects);
				render(&mut renderer, &mut pools, &scene, &camera);
			},
			Event::RedrawEventsCleared => {
				window.request_redraw();
			},
			Event::RedrawRequested(_) => {
				update(&mut pools, &scene, &objects);
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
