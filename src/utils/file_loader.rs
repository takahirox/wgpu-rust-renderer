pub struct FileLoader {
}

// Non-Wasm

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;

#[cfg(not(target_arch = "wasm32"))]
impl FileLoader {
	pub async fn open(file_path: &str) -> File {
		File::open(file_path).unwrap()
	}
}

// Wasm

#[cfg(target_arch = "wasm32")]
use {
	std::io::Cursor,
	wasm_bindgen::JsCast,
	wasm_bindgen_futures::JsFuture,
	web_sys::{
		Request,
		RequestInit,
		RequestMode,
		Response,
	},
};

// @TODO: Proper error handling
#[cfg(target_arch = "wasm32")]
impl FileLoader {
	pub async fn open(file_path: &str) -> Cursor<Vec<u8>> {
		let result = fetch_as_binary(file_path).await.unwrap();
		Cursor::new(result)
	}
}

// @TODO: Proper error handling
#[cfg(target_arch = "wasm32")]
pub async fn fetch_as_binary(url: &str) -> Result<Vec<u8>, String> {
	let mut opts = RequestInit::new();
	opts.method("GET");
	opts.mode(RequestMode::Cors); // @TODO: Should be able to opt-out

	let request = match Request::new_with_str_and_init(&url, &opts) {
		Ok(request) => request,
		Err(_e) => return Err("Failed to create request".to_string()),
	};

	let window = web_sys::window().unwrap();
	let response = match JsFuture::from(window.fetch_with_request(&request)).await {
		Ok(response) => response,
		Err(_e) => return Err("Failed to fetch".to_string()),
	};

	let response: Response = match response.dyn_into() {
		Ok(response) => response,
		Err(_e) => return Err("Failed to dyn_into Response".to_string()),
	};

	let buffer = match response.array_buffer() {
		Ok(buffer) => buffer,
		Err(_e) => return Err("Failed to get as array buffer".to_string()),
	};

	let buffer = match JsFuture::from(buffer).await {
		Ok(buffer) => buffer,
		Err(_e) => return Err("Failed to ...?".to_string()),
	};

	Ok(js_sys::Uint8Array::new(&buffer).to_vec())
}
