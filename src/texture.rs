use std::borrow::Cow;
use std::path::Path;
use image::{ColorType, ImageFormat};
pub use crate::raw::texture as raw;

pub use raw::*;

use crate::gl;
use crate::state::GraphicsContext;

#[derive(Copy, Clone)]
pub struct Mipmap {
    pub levels: i32
}

pub struct TextureParameters {
    pub wrap_s: TextureWrap,
    pub wrap_t: TextureWrap,
    pub wrap_r: TextureWrap,
    pub min_filter: MinFilter,
    pub mag_filter: MagFilter,
    pub mipmap: Option<Mipmap>
}

impl Default for TextureParameters {
    fn default() -> Self {
        Self {
            wrap_s: TextureWrap::ClampToBorder,
            wrap_t: TextureWrap::ClampToBorder,
            wrap_r: TextureWrap::ClampToBorder,
            min_filter: MinFilter::Nearest,
            mag_filter: MagFilter::Nearest,
            mipmap: None,
        }
    }
}

mod gamma {
    include!(concat!(env!("OUT_DIR"), "/gamma.rs"));

    pub fn get(i: u32) -> f32 {
        GAMMA[i as usize & 255]
    }
}

#[inline(always)]
fn to_exp(sum: f32, n: usize) -> u8 {
    let r = (sum / n as f32).powf(5.0 / 11.0);
    (r * 255.0) as u8
}

fn blend_component<const N: usize>(parts: [u32; N], shift: u32) -> u8 {
    let mut sum = 0.0;
    for p in parts {
        sum += gamma::get(p >> shift);
    }
    to_exp(sum, N)
}

fn blend_colors<const N: usize>(colors: [u32; N], has_alpha: bool) -> u32 {
    if has_alpha {
        let mut alpha = 0.0;
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        for c in colors {
            if c >> 24 != 0 {
                alpha += gamma::get(c >> 24);
                red += gamma::get(c >> 16);
                green += gamma::get(c >> 8);
                blue += gamma::get(c);
            }
        }
        let a = to_exp(alpha, N);
        let r = to_exp(red, N);
        let g = to_exp(green, N);
        let b = to_exp(blue, N);
        u32::from_be_bytes([if a < 96 { 0 } else { a }, r, g, b])
    } else {
        let a = blend_component(colors, 24);
        let r = blend_component(colors, 16);
        let g = blend_component(colors, 8);
        let b = blend_component(colors, 0);
        u32::from_be_bytes([a, r, g, b])
    }
}

fn generate_mipmap(levels: u32, width: u32, data: Option<&[u8]>) -> Vec<Cow<[u8]>> {
    let mut layers = Vec::with_capacity(levels as usize + 1);
    let Some(data) = data else {
        return layers;
    };

    layers.push(Cow::Borrowed(data));

    let data = bytemuck::cast_slice::<_, u32>(data);

    let alpha = data.iter().any(|&b| (b >> 24) == 0);

    for level in 1..=levels {
        let previous = bytemuck::cast_slice::<_, u32>(&layers[level as usize - 1]);
        let mut current = vec![0; previous.len() >> 2];
        let w = width >> level;
        if w > 0 {
            let h = current.len() as u32 / w;
            let mi = w << 1;

            for x in 0..w {
                for y in 0..h {
                    let oi = 2 * (x + y * mi) as usize;
                    let i = (x + y * w) as usize;
                    current[i] = blend_colors([
                        previous[oi],
                        previous[oi + 1],
                        previous[oi + mi as usize],
                        previous[oi + 1 + mi as usize],
                    ], alpha);
                }
            }
        }

        unsafe {
            let (ptr, len, capacity) = current.into_raw_parts();
            let layer = Vec::from_raw_parts(ptr as *mut u8, len * 4, capacity * 4);
            layers.push(Cow::Owned(layer));
        }
    }

    layers
}

pub fn allocate<C: Into<GraphicsContext>>(width: u32, height: u32, data: Option<&[u8]>, params: &TextureParameters, c: C) -> TextureId {
    let texture = c.into().gen_texture().expect("Unable to allocate texture");
    let bound = texture.bind();
    bound.wrap_s(params.wrap_s);
    bound.wrap_t(params.wrap_t);
    bound.wrap_r(params.wrap_r);
    bound.min_filter(params.min_filter);
    bound.mag_filter(params.mag_filter);
    let levels = params.mipmap.map(|m| m.levels).unwrap_or_default();
    if levels >= 0 {
        bound.max_level(levels);
        bound.min_lod(0);
        bound.max_lod(levels);
        bound.lod_bias(0.0);

        let layers = generate_mipmap(levels as u32, width, data);

        for i in 0..=levels as u32 {
            bound.set_image(i, InternalTextureFormat::RGBA8, width >> i, height >> i, UploadPixelFormat::RGBA, layers.get(i as usize).map(Cow::as_ref));
        }
    } else {
        bound.set_image(0, InternalTextureFormat::RGBA8, width, height, UploadPixelFormat::RGBA, data);
    }
    drop(bound);
    texture
}

pub fn from_memory<C: Into<GraphicsContext>>(data: &[u8], format: ImageFormat, params: &TextureParameters, c: C) -> TextureId {
    let source = image::load_from_memory_with_format(data, format)
        .expect("Failed to load texture from memory")
        .into_rgba8();
    allocate(source.width(), source.height(), Some(source.as_raw()), params, c)
}

pub fn from_file<P: AsRef<Path>, C: Into<GraphicsContext>>(path: P, params: &TextureParameters, c: C) -> TextureId {
    let source = image::open(path)
        .expect("Failed to open texture file")
        .into_rgba8();
    allocate(source.width(), source.height(), Some(source.as_raw()), params, c)
}

pub fn download<P: AsRef<Path>>(texture: &TextureGuard, format: DownloadPixelFormat, layer: u32, path: P) {
    unsafe {
        gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        let width = texture.get_layer_width(layer);
        let height = texture.get_layer_height(layer);
        let mut output = vec![0u32; (width * height) as usize];

        texture.get_image(layer, format, width, height, &mut output);

        image::save_buffer_with_format(path, bytemuck::cast_slice(&output), width, height, ColorType::Rgba8, ImageFormat::Png)
            .expect("Failed to download texture from memory");
    }
}