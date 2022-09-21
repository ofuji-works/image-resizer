extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

use image::*;
use js_sys::*;
use std::io::Cursor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn time(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn timeEnd(s: &str);
}

#[wasm_bindgen]
pub fn image_resize(
    array_buffer: Uint8Array,
    width: usize,
    height: usize,
    format: &str,
) -> Uint8Array {
    console_error_panic_hook::set_once();
    let buffer = array_buffer.to_vec();
    let img = load_from_memory(&buffer).expect("Error occurs at load image from buffer.");
    let resized = img.resize_exact(width as u32, height as u32, imageops::FilterType::Triangle);
    let result = save_to_buffer(resized, format);

    Uint8Array::new(&unsafe { Uint8Array::view(&result) }.into())
}

fn save_to_buffer(img: DynamicImage, format: &str) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let fmt = match format {
        "png" => ImageOutputFormat::Png,
        "gif" => ImageOutputFormat::Gif,
        "bmp" => ImageOutputFormat::Bmp,
        "jpg" => ImageOutputFormat::Jpeg(80),
        unsupport => ImageOutputFormat::Unsupported(String::from(unsupport)),
    };

    let mut result: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut result), fmt)
        .expect("Error occurs at save image from buffer.");

    result
}
