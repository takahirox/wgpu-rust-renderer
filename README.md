[![Build Status](https://travis-ci.org/takahirox/wgpu_rust_renderer.svg?branch=main)](https://travis-ci.org/takahirox/wgpu_rust_renderer)
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

T.B.D.

## How to import

The library is released at [crates.io](https://crates.io/crates/wgpu_rust_renderer). Add the following line into Cargo.toml of your Rust project.

```
[dependencies]
wgpu_rust_renderer = "0.0.1"
```

And add the following lines in your Rust code to import the library.

```Rust
use wgpu_rust_renderer::{
  scene::{
    attribute::AttributeManager,
    geometry::Geometry,
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
