use std::num::NonZeroU32;
use std::rc::Rc;
use bytemuck::{AnyBitPattern, NoUninit, Zeroable};
use crate::gl;
use crate::state::GraphicsContext;
use crate::tessellator::Color;
use super::buffer::DrawTarget;
use super::GLenum;

#[repr(transparent)]
pub struct TextureId(pub NonZeroU32, pub(crate) GraphicsContext);

impl AsRef<TextureId> for TextureId {
    #[inline(always)]
    fn as_ref(&self) -> &TextureId {
        self
    }
}

impl Drop for TextureId {
    #[inline(always)]
    fn drop(&mut self) {
        let id = self.0.get();
        unsafe {
            gl::DeleteTextures(1, &id)
        }
    }
}

pub struct TextureGuard<'texture> {
    texture: &'texture TextureId
}

impl<'texture> TextureGuard<'texture> {
    #[inline(always)]
    pub(super) unsafe fn id(&self) -> NonZeroU32 {
        self.texture.0
    }

    #[inline(always)]
    pub fn context(&self) -> &GraphicsContext {
        &self.texture.1
    }

    #[inline(always)]
    unsafe fn parameter_i32(&self, parameter: GLenum, value: i32) {
        gl::TexParameteri(gl::TEXTURE_2D, parameter, value as _)
    }

    #[inline(always)]
    unsafe fn parameter_f32(&self, parameter: GLenum, value: f32) {
        gl::TexParameterf(gl::TEXTURE_2D, parameter, value as _)
    }

    #[inline(always)]
    unsafe fn get_level_parameter_i32(&self, level: u32, parameter: GLenum, output: &mut [i32]) {
        gl::GetTexLevelParameteriv(gl::TEXTURE_2D, level as _, parameter, output.as_mut_ptr())
    }

    #[inline(always)]
    pub fn get_layer_width(&self, level: u32) -> u32 {
        let mut w = 0;
        unsafe {
            self.get_level_parameter_i32(level, gl::TEXTURE_WIDTH, std::slice::from_mut(&mut w));
        }
        w as _
    }

    #[inline(always)]
    pub fn get_layer_height(&self, level: u32) -> u32 {
        let mut h = 0;
        unsafe {
            self.get_level_parameter_i32(level, gl::TEXTURE_HEIGHT, std::slice::from_mut(&mut h));
        }
        h as _
    }

    #[inline(always)]
    pub fn wrap_s(&self, wrap: TextureWrap) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_WRAP_S, wrap as _)
        }
    }

    #[inline(always)]
    pub fn wrap_t(&self, wrap: TextureWrap) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_WRAP_T, wrap as _)
        }
    }

    #[inline(always)]
    pub fn wrap_r(&self, wrap: TextureWrap) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_WRAP_R, wrap as _)
        }
    }

    #[inline(always)]
    pub fn min_filter(&self, filter: MinFilter) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_MIN_FILTER, filter as _)
        }
    }

    #[inline(always)]
    pub fn mag_filter(&self, filter: MagFilter) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_MAG_FILTER, filter as _)
        }
    }

    #[inline(always)]
    pub fn max_level(&self, level: i32) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_MAX_LEVEL, level as _)
        }
    }

    #[inline(always)]
    pub fn min_lod(&self, lod: i32) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_MIN_LOD, lod as _)
        }
    }

    #[inline(always)]
    pub fn max_lod(&self, lod: i32) {
        unsafe {
            self.parameter_i32(gl::TEXTURE_MAX_LOD, lod as _)
        }
    }

    #[inline(always)]
    pub fn lod_bias(&self, bias: f32) {
        unsafe {
            self.parameter_f32(gl::TEXTURE_LOD_BIAS, bias as _)
        }
    }

    #[inline(always)]
    pub fn copy_sub_image<T: DrawTarget>(&self, level: u32, x_offset: u32, y_offset: u32, x: u32, y: u32, w: u32, h: u32, source: &T) {
        let _bound = source.bind();
        unsafe {
            gl::CopyTexSubImage2D(gl::TEXTURE_2D, level as _, x_offset as _, y_offset as _, x as _, y as _, w as _, h as _);
        }
    }

    #[inline(always)]
    pub fn set_sub_image<P>(&self, level: u32, x: u32, y: u32, width: u32, height: u32, format: UploadPixelFormat, pixels: Option<&[P]>) where P: Pixel {
        let bytes: Option<&[u8]> = pixels.map(bytemuck::cast_slice);
        if let Some(bytes) = bytes {
            let size = P::size() * (width * height) as usize;
            assert_eq!(bytes.len(), size);
        }
        unsafe {
            let ptr = bytes.map_or_else(std::ptr::null, |v| v.as_ptr() as _);
            gl::TexSubImage2D(gl::TEXTURE_2D, level as _, x as _, y as _, width as _, height as _, format as _, P::gl_type(), ptr);
        }
    }

    #[inline(always)]
    pub fn set_image<P>(&self, level: u32, internal_format: InternalTextureFormat, width: u32, height: u32, format: UploadPixelFormat, pixels: Option<&[P]>) where P: Pixel {
        let bytes: Option<&[u8]> = pixels.map(bytemuck::cast_slice);
        if let Some(bytes) = bytes {
            let size = P::size() * (width * height) as usize;
            assert_eq!(bytes.len(), size);
        }
        unsafe {
            let ptr = bytes.map_or_else(std::ptr::null, |v| v.as_ptr() as _);
            gl::TexImage2D(gl::TEXTURE_2D, level as _, internal_format as _, width as _ , height as _, 0, format as _, P::gl_type(), ptr);
        }
    }

    #[inline(always)]
    pub fn get_image<P>(&self, level: u32, format: DownloadPixelFormat, width: u32, height: u32, pixels: &mut [P]) where P: Pixel {
        let bytes: &mut [u8] = bytemuck::cast_slice_mut(pixels);
        let size = P::size() * (width * height) as usize;
        assert_eq!(bytes.len(), size);
        unsafe {
            gl::GetTexImage(gl::TEXTURE_2D, level as _, format as _, P::gl_type(), bytes.as_mut_ptr().cast());
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UploadPixelFormat {
    Red = gl::RED,
    RG = gl::RG,
    RGB = gl::RGB,
    BGR = gl::BGR,
    RGBA = gl::RGBA,
    BGRA = gl::BGRA,
    RedInteger = gl::RED_INTEGER,
    RGInteger = gl::RG_INTEGER,
    RGBInteger = gl::RGB_INTEGER,
    BGRInteger = gl::BGR_INTEGER,
    RGBAInteger = gl::RGBA_INTEGER,
    StencilIndex = gl::STENCIL_INDEX,
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DownloadPixelFormat {
    StencilIndex = gl::STENCIL_INDEX,
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL,
    Red = gl::RED,
    Green = gl::GREEN,
    Blue = gl::BLUE,
    RG = gl::RG,
    RGB = gl::RGB,
    RGBA = gl::RGBA,
    BGR = gl::BGR,
    BGRA = gl::BGRA,
    RedInteger = gl::RED_INTEGER,
    GreenInteger = gl::GREEN_INTEGER,
    BlueInteger = gl::BLUE_INTEGER,
    RGInteger = gl::RG_INTEGER,
    RGBInteger = gl::RGB_INTEGER,
    RGBAInteger = gl::RGBA_INTEGER,
    BGRInteger = gl::BGR_INTEGER,
    BGRAInteger = gl::BGRA_INTEGER
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BaseTextureFormat {
    Red = gl::RED,
    RG = gl::RG,
    RGB = gl::RGB,
    RGBA = gl::RGBA
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum InternalTextureFormat {
    R8 = gl::R8,
    R8SNorm = gl::R8_SNORM,
    R16 = gl::R16,
    R16SNorm = gl::R16_SNORM,
    RG8 = gl::RG8,
    RG8SNorm = gl::RG8_SNORM,
    RG16 = gl::RG16,
    RG16SNorm = gl::RG16_SNORM,
    R3G3B2 = gl::R3_G3_B2,
    RGB4 = gl::RGB4,
    RGB5 = gl::RGB5,
    RGB8 = gl::RGB8,
    RGB8SNorm = gl::RGB8_SNORM,
    RGB10 = gl::RGB10,
    RGB12 = gl::RGB12,
    RGB16SNorm = gl::RGB16_SNORM,
    RGBA2 = gl::RGBA2,
    RGBA4 = gl::RGBA4,
    RGB5A1 = gl::RGB5_A1,
    RGBA8 = gl::RGBA8,
    RGBA8SNorm = gl::RGBA8_SNORM,
    RGB10A2 = gl::RGB10_A2,
    RGB10A2UI = gl::RGB10_A2UI,
    RGBA12 = gl::RGBA12,
    RGBA16 = gl::RGBA16,
    SRGB8 = gl::SRGB8,
    SRGB8Alpha8 = gl::SRGB8_ALPHA8,
    R16F = gl::R16F,
    RG16F = gl::RG16F,
    RGB16F = gl::RGB16F,
    RGBA16F = gl::RGBA16F,
    R32F = gl::R32F,
    RG32F = gl::RG32F,
    RGB32F = gl::RGB32F,
    RGBA32F = gl::RGBA32F,
    R11FG11FB10F = gl::R11F_G11F_B10F,
    RGB9E5 = gl::RGB9_E5,
    R8I = gl::R8I,
    R8UI = gl::R8UI,
    R16I = gl::R16I,
    R16UI = gl::R16UI,
    R32I = gl::R32I,
    R32UI = gl::R32UI,
    RG8I = gl::RG8I,
    RG8UI = gl::RG8UI,
    RG16I = gl::RG16I,
    RG16UI = gl::RG16UI,
    RG32I = gl::RG32I,
    RG32UI = gl::RG32UI,
    RGB8I = gl::RGB8I,
    RGB8UI = gl::RGB8UI,
    RGB16I = gl::RGB16I,
    RGB16UI = gl::RGB16UI,
    RGB32I = gl::RGB32I,
    RGB32UI = gl::RGB32UI,
    RGBA8I = gl::RGBA8I,
    RGBA8UI = gl::RGBA8UI,
    RGBA16I = gl::RGBA16I,
    RGBA16UI = gl::RGBA16UI,
    RGBA32I = gl::RGBA32I,
    RGBA32UI = gl::RGBA32UI,
}

impl InternalTextureFormat {
    pub fn base(&self) -> BaseTextureFormat {
        match self {
            Self::R8 => BaseTextureFormat::Red,
            Self::R8SNorm => BaseTextureFormat::Red,
            Self::R16 => BaseTextureFormat::Red,
            Self::R16SNorm => BaseTextureFormat::Red,
            Self::RG8 => BaseTextureFormat::RG,
            Self::RG8SNorm => BaseTextureFormat::RG,
            Self::RG16 => BaseTextureFormat::RG,
            Self::RG16SNorm => BaseTextureFormat::RG,
            Self::R3G3B2 => BaseTextureFormat::RGB,
            Self::RGB4 => BaseTextureFormat::RGB,
            Self::RGB5 => BaseTextureFormat::RGB,
            Self::RGB8 => BaseTextureFormat::RGB,
            Self::RGB8SNorm => BaseTextureFormat::RGB,
            Self::RGB10 => BaseTextureFormat::RGB,
            Self::RGB12 => BaseTextureFormat::RGB,
            Self::RGB16SNorm => BaseTextureFormat::RGB,
            Self::RGBA2 => BaseTextureFormat::RGB,
            Self::RGBA4 => BaseTextureFormat::RGB,
            Self::RGB5A1 => BaseTextureFormat::RGBA,
            Self::RGBA8 => BaseTextureFormat::RGBA,
            Self::RGBA8SNorm => BaseTextureFormat::RGBA,
            Self::RGB10A2 => BaseTextureFormat::RGBA,
            Self::RGB10A2UI => BaseTextureFormat::RGBA,
            Self::RGBA12 => BaseTextureFormat::RGBA,
            Self::RGBA16 => BaseTextureFormat::RGBA,
            Self::SRGB8 => BaseTextureFormat::RGB,
            Self::SRGB8Alpha8 => BaseTextureFormat::RGBA,
            Self::R16F => BaseTextureFormat::Red,
            Self::RG16F => BaseTextureFormat::RG,
            Self::RGB16F => BaseTextureFormat::RGB,
            Self::RGBA16F => BaseTextureFormat::RGBA,
            Self::R32F => BaseTextureFormat::Red,
            Self::RG32F => BaseTextureFormat::RG,
            Self::RGB32F => BaseTextureFormat::RGB,
            Self::RGBA32F => BaseTextureFormat::RGBA,
            Self::R11FG11FB10F => BaseTextureFormat::RGB,
            Self::RGB9E5 => BaseTextureFormat::RGB,
            Self::R8I => BaseTextureFormat::Red,
            Self::R8UI => BaseTextureFormat::Red,
            Self::R16I => BaseTextureFormat::Red,
            Self::R16UI => BaseTextureFormat::Red,
            Self::R32I => BaseTextureFormat::Red,
            Self::R32UI => BaseTextureFormat::Red,
            Self::RG8I => BaseTextureFormat::RG,
            Self::RG8UI => BaseTextureFormat::RG,
            Self::RG16I => BaseTextureFormat::RG,
            Self::RG16UI => BaseTextureFormat::RG,
            Self::RG32I => BaseTextureFormat::RG,
            Self::RG32UI => BaseTextureFormat::RG,
            Self::RGB8I => BaseTextureFormat::RGB,
            Self::RGB8UI => BaseTextureFormat::RGB,
            Self::RGB16I => BaseTextureFormat::RGB,
            Self::RGB16UI => BaseTextureFormat::RGB,
            Self::RGB32I => BaseTextureFormat::RGB,
            Self::RGB32UI => BaseTextureFormat::RGB,
            Self::RGBA8I => BaseTextureFormat::RGBA,
            Self::RGBA8UI => BaseTextureFormat::RGBA,
            Self::RGBA16I => BaseTextureFormat::RGBA,
            Self::RGBA16UI => BaseTextureFormat::RGBA,
            Self::RGBA32I => BaseTextureFormat::RGBA,
            Self::RGBA32UI => BaseTextureFormat::RGBA,
        }
    }
}

#[repr(u32)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MinFilter {
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
    #[default]
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
}

#[repr(u32)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum MagFilter {
    Nearest = gl::NEAREST,
    #[default]
    Linear = gl::LINEAR,
}

#[repr(u32)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TextureWrap {
    ClampToEdge = gl::CLAMP_TO_EDGE,
    ClampToBorder = gl::CLAMP_TO_BORDER,
    MirroredRepeat = gl::MIRRORED_REPEAT,
    #[default]
    Repeat = gl::REPEAT,
    MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE,
}

pub unsafe trait Pixel: NoUninit + AnyBitPattern {
    fn gl_type() -> GLenum;

    fn size() -> usize;
}

unsafe impl NoUninit for Color {}

unsafe impl AnyBitPattern for Color {}

unsafe impl Zeroable for Color {}

unsafe impl Pixel for Color {
    #[inline(always)]
    fn gl_type() -> GLenum {
        gl::UNSIGNED_INT_8_8_8_8_REV
    }

    #[inline(always)]
    fn size() -> usize {
        4
    }
}

unsafe impl Pixel for u32 {
    #[inline(always)]
    fn gl_type() -> GLenum {
        gl::UNSIGNED_BYTE
    }

    #[inline(always)]
    fn size() -> usize {
        4
    }
}

impl<'texture> Drop for TextureGuard<'texture> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0)
        }
    }
}

pub trait Texture<'a> {
    fn bind(self) -> Rc<TextureGuard<'a>>;
}

impl<'a> Texture<'a> for &'a TextureId {
    #[inline(always)]
    fn bind(self) -> Rc<TextureGuard<'a>> {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0.get());
        }
        Rc::new(TextureGuard {
            texture: self
        })
    }
}

impl<'a> Texture<'a> for &'a mut TextureId {
    #[inline(always)]
    fn bind(self) -> Rc<TextureGuard<'a>> {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0.get());
        }
        Rc::new(TextureGuard {
            texture: self
        })
    }
}