RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --no-default-features --release --lib --target wasm32-unknown-unknown 
wasm-bindgen ./target/wasm32-unknown-unknown/release/wgpu_rust_renderer_example_face_culling.wasm --out-dir ./ --target web --no-typescript
