use std::path::Path;
use image::{ColorType, ImageFormat};
pub use crate::raw::texture as raw;

pub use raw::*;

use crate::gl;
use crate::state::GraphicsContext;

pub fn allocate<C: Into<GraphicsContext>>(width: u32, height: u32, c: C) -> TextureId {
    let texture = c.into().gen_texture().expect("Unable to allocate texture");
    let bound = texture.bind();
    bound.set_image::<u32>(0, InternalTextureFormat::RGBA8, width, height, UploadPixelFormat::BGRA, None);
    drop(bound);
    texture
}

pub fn from_bytes<C: Into<GraphicsContext>>(width: u32, height: u32, data: &[u8], c: C) -> TextureId {
    let c = c.into();
    let texture = c.gen_texture().expect("Unable to allocate texture");
    let bound = texture.bind();
    bound.wrap_s(TextureWrap::ClampToBorder);
    bound.wrap_t(TextureWrap::ClampToBorder);
    bound.min_filter(MinFilter::Nearest);
    bound.mag_filter(MagFilter::Nearest);
    bound.set_image(0, InternalTextureFormat::RGBA8, width, height, UploadPixelFormat::RGBA, Some(data));
    drop(bound);
    texture
}

pub fn from_memory<C: Into<GraphicsContext>>(data: &[u8], format: ImageFormat, c: C) -> TextureId {
    let source = image::load_from_memory_with_format(data, format)
        .expect("Failed to load texture from memory")
        .into_rgba8();
    from_bytes(source.width(), source.height(), source.as_raw(), c)
}

pub fn from_file<P: AsRef<Path>, C: Into<GraphicsContext>>(path: P, c: C) -> TextureId {
    let source = image::open(path)
        .expect("Failed to open texture file")
        .into_rgba8();
    from_bytes(source.width(), source.height(), source.as_raw(), c)
}

pub fn download<P: AsRef<Path>>(texture: &TextureGuard, format: DownloadPixelFormat, layer: u32, path: P) {
    unsafe {
        gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        let width = texture.get_layer_width(layer);
        let height = texture.get_layer_height(layer);
        eprintln!("w = {}, h = {}", width, height);
        let mut output = vec![0u32; (width * height) as usize];

        texture.get_image(layer, format, width, height, &mut output);

        image::save_buffer_with_format(path, bytemuck::cast_slice(&output), width, height, ColorType::Rgba8, ImageFormat::Png)
            .expect("Failed to download texture from memory");
    }
}