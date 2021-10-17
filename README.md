[![Build Status](https://travis-ci.org/takahirox/wgpu-rust-renderer.svg?branch=main)](https://travis-ci.org/takahirox/wgpu-rust-renderer)
[![Crate](https://img.shields.io/crates/v/wgpu_rust_renderer.svg)](https://crates.io/crates/wgpu_rust_renderer)

# wgpu-rust-renderer

`wgpu-rust-renderer` is a tiny [WebGPU](https://www.w3.org/TR/webgpu/) Renderer written in Rust.

## Demo

[Online WebAssembly Demo](https://takahirox.github.io/wgpu-rust-renderer/web/examples/#triangle)

Rust code is compiled to WebAssembly with [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) and it runs even in web browsers.

## Screenshots

Desktop application.

<img src="./screenshots/desktop.png" width="640" alt="Desktop app">

Web application.

<img src="./screenshots/web.png" width="640" alt="Web app">

## Features

* Tiny WebGPU Renderling library
* Easy to use
* Memory safe with Rust
* Web application compatible by compiling to WebAssembly

## Documents

T.B.D.

## Sample Code

```rust
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
    index::IndexManager,
    mesh::Mesh,
    scene::Scene,
    texture::TextureManager,
  },
  utils::geometry_helper::GeometryHelper,
};

fn create_scene(window: &Window) -> Scene {
  let mut scene = Scene::new();
  let mut attribute_manager = AttributeManager::new();
  let mut index_manager = IndexManager::new();
  let mut texture_manager = TextureManager::new();

  let geometry = GeometryHelper::create_triangle(
    &mut attribute_manager,
    &mut index_manager,
    1.0,
    1.0,
  );

  let texture = texture_manager.create_dummy();
  let mut material = Material::new();
  Color::set(material.borrow_color_mut(), 1.0, 0.0, 0.0);
  material.set_texture(Some(texture));

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
```

## How to import

The library is released at [crates.io](https://crates.io/crates/wgpu_rust_renderer). Add the following line into Cargo.toml of your Rust project.

```
[dependencies]
wgpu_rust_renderer = "0.0.1"
```

And add the following lines in your Rust code to import the library.

```Rust
use wgpu_rust_renderer::{
  material::material::Material,
  scene::{
    attribute::AttributeManager,
    camera::PerspectiveCamera,
    geometry::Geometry,
    index::IndexManager,
    mesh::Mesh,
    scene::Scene,
  },
  web::wgpu_web_renderer::WGPUWebRenderer, // for web
  renderer::wgpu_renderer::WGPURenderer, // for others
};
```

## How to build the library locally

```sh
$ git clone https://github.com/takahirox/wgpu-rust-renderer.git
$ cd wgpu-rust-renderer
$ cargo build
```

## How to run desktop examples locally

```sh
$ cd wgpu-rust-renderer
$ cargo run --example example_name
```

## How to run web examles locally

Prerequirements
- Install [wasm-bindgen client](https://rustwasm.github.io/docs/wasm-bindgen/)
- Install Rust wasm32-unknown-unknown target with `$ rustup target add wasm32-unknown-unknown`
- Install `http-server` with `$ npm install -g http-server`, or other local servers

```sh
$ cd wgpu-rust-renderer/web
$ bash build_examples.sh
$ http-server . -p 8080 -c-1
# Access http://localhost:8080/examples/index.html on your web browser
```

## How to run tests

T.B.D.
