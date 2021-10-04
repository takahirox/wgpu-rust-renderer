use std::ops::{Deref, DerefMut};
use winit::window::Window;
use crate::renderer::wgpu_renderer::WGPURenderer;

pub struct WGPUWebRenderer {
	canvas: web_sys::HtmlCanvasElement,
	renderer: WGPURenderer,
}

// Using Deref/DerefMut for inheritance may be a bad design

impl Deref for WGPUWebRenderer {
	type Target = WGPURenderer;
	fn deref(&self) -> &WGPURenderer {
		&self.renderer
	}
}

impl DerefMut for WGPUWebRenderer {
	fn deref_mut(&mut self) -> &mut WGPURenderer {
		&mut self.renderer
	}
}

impl WGPUWebRenderer {
	pub async fn new(window: &Window, canvas: web_sys::HtmlCanvasElement) -> WGPUWebRenderer {
		WGPUWebRenderer {
			canvas: canvas,
			renderer: WGPURenderer::new(window).await,
		}
	}

	pub fn borrow_canvas(&self) -> &web_sys::HtmlCanvasElement {
		&self.canvas
	}

	pub fn set_size(
		&mut self,
		width: f64,
		height: f64,
	) {
		WGPURenderer::set_size(&mut self.renderer, width, height);
		update_canvas_size(
			&self.canvas,
			self.renderer.get_size(),
			self.renderer.get_pixel_ratio()
		);
	}

	pub fn set_pixel_ratio(
		&mut self,
		_pixel_ratio: f64,
	) {
		// I don't know thy but pixel_ratio parameter needs to be 1.0 for Web.
		// Otherwise, crashes with
		//   Attachment size mismatch
		//   - While encoding BeginRenderPass([RenderPassDescriptor]).
		// error.
		// @TODO: Fix the root issue
		let pixel_ratio = 1.0;

		WGPURenderer::set_pixel_ratio(&mut self.renderer, pixel_ratio);
		update_canvas_size(
			&self.canvas,
			self.renderer.get_size(),
			self.renderer.get_pixel_ratio()
		);
	}
}

fn update_canvas_size(
	canvas: &web_sys::HtmlCanvasElement,
	(width, height): (f64, f64),
	pixel_ratio: f64
) {
	canvas.set_width((width * pixel_ratio) as u32);
	canvas.set_height((height * pixel_ratio) as u32);
	canvas.style().set_property("width", &((width as u32).to_string() + "px")).unwrap();
	canvas.style().set_property("height", &((height as u32).to_string() + "px")).unwrap();
}
