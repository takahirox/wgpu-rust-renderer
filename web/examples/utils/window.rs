use wasm_bindgen::{
	JsCast,
	prelude::*,
};
use winit::{
	event_loop::EventLoop,
};

// Window and DOM element helpers

pub fn get_window_inner_size() -> (f64, f64) {
	let window = web_sys::window().unwrap();
	(
		window.inner_width().unwrap().as_f64().unwrap(),
		window.inner_height().unwrap().as_f64().unwrap()
	)
}

pub fn get_window_device_pixel_ratio() -> f64 {
	let window = web_sys::window().unwrap();
	window.device_pixel_ratio()
}

pub fn create_window(event_loop: &EventLoop<()>) -> std::rc::Rc<winit::window::Window> {
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
