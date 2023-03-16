use std::ffi::c_void;
use cgmath::{Matrix4, };
use crate::framebuffer::DrawTarget;
use crate::state::blend::Blend;
use crate::state::draw::DrawMode;
use crate::state::shade::ShadeModel;
use crate::texture::TextureGuard;
use self::formats::{PositionColor, PositionTex};

use super::gl;
use super::gl::types::GLenum;

pub struct Color(u8, u8, u8, u8);

impl From<u32> for Color {
    #[inline(always)]
    fn from(value: u32) -> Self {
        let [r, g, b, a] = value.to_be_bytes();
        Self(r, g, b, a)
    }
}

impl From<[u8; 4]> for Color {
    #[inline(always)]
    fn from(value: [u8; 4]) -> Self {
        let [r, g, b, a] = value;
        Self(r, g, b, a)
    }
}

impl Element for Color {
    #[inline(always)]
    fn size() -> usize {
        16
    }

    #[inline(always)]
    fn len() -> usize {
        4
    }

    #[inline(always)]
    fn gl() -> GLenum {
        gl::UNSIGNED_BYTE
    }
}

pub mod formats {
    use super::Color;
    use super::ElementUsage;
    use super::Vertex;
    use super::Element;
    use crate::gl;
    use crate::state::draw::DrawMode;
    use cgmath::{Matrix4, Matrix};

    macro_rules! vertex {
        ($name:ident($(#[$usage:expr] $field:ident : $ty:ty),*)) => {
            #[repr(C)]
            pub struct $name {
                $(pub $field: $ty),+
            }

            impl Vertex for $name {
                #[inline(always)]
                fn upload(data: &[$name], mode: DrawMode, matrix: &Matrix4<f32>) {
                    if data.len() > 0 {
                        let size = std::mem::size_of::<Self>();
                        $(
                            let offset = field_offset::offset_of!(Self => $field);
                            let len = <$ty as Element>::len();
                            let gl = <$ty as Element>::gl();
                            let usage = $usage;
                            usage.begin(len as _, gl, size as i32, offset.apply_ptr(data.as_ptr()).cast());
                        )*
                        unsafe {
                            gl::MatrixMode(gl::MODELVIEW);
                            gl::PushMatrix();
                            gl::LoadIdentity();
                            gl::MultMatrixf(matrix.as_ptr());
                            gl::DrawArrays(mode as _, 0, data.len() as i32);
                            gl::PopMatrix();
                        }
                        $(
                            $usage.end();
                        )*
                    }
                }
            }

            impl $name {
                pub fn new($($field: impl Into<$ty>),*) -> Self {
                    Self {
                        $($field: $field.into()),*
                    }
                }
            }
        };
    }

    vertex!(Position(
        #[ElementUsage::Position]
        pos: [f32; 3]
    ));
    vertex!(PositionColor(
        #[ElementUsage::Position]
        pos: [f32; 3],
        #[ElementUsage::Color]
        color: Color
    ));
    vertex!(PositionTex(
        #[ElementUsage::Position]
        pos: [f32; 3],
        #[ElementUsage::Texture(0)]
        uv: [f32; 2]
    ));
    vertex!(PositionNormal(
        #[ElementUsage::Position]
        pos: [f32; 3],
        #[ElementUsage::Normal]
        normal: [i8; 3]
    ));
    vertex!(PositionTexColor(
        #[ElementUsage::Position]
        pos: [f32; 3],
        #[ElementUsage::Texture(0)]
        uv: [f32; 2],
        #[ElementUsage::Color]
        color: Color
    ));
}

pub trait Vertex: Sized {
    fn upload(data: &[Self], mode: DrawMode, matrix: &Matrix4<f32>);
}

trait Element: Sized {
    #[inline(always)]
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }

    fn len() -> usize {
        1
    }

    fn gl() -> GLenum;
}

impl<E, const N: usize> Element for [E; N] where E: Element {
    #[inline(always)]
    fn len() -> usize {
        N
    }

    #[inline(always)]
    fn gl() -> GLenum {
        E::gl()
    }
}

macro_rules! element {
    ($ty:ty, $var:ident) => {
        impl Element for $ty {
            #[inline(always)]
            fn gl() -> GLenum {
                gl::$var
            }
        }
    };
}

element!(f32, FLOAT);
element!(f64, DOUBLE);
element!(u8, UNSIGNED_BYTE);
element!(i8, BYTE);
element!(u16, UNSIGNED_SHORT);
element!(i16, SHORT);
element!(u32, UNSIGNED_INT);
element!(i32, INT);

#[derive(Debug, Copy, Clone, Hash, PartialOrd, PartialEq)]
pub enum ElementUsage {
    Position,
    Normal,
    Color,
    Texture(u32),
    Generic(u32),
}

impl ElementUsage {
    #[inline(always)]
    fn begin(&self, count: i32, ty: GLenum, stride: i32, ptr: *const c_void) {
        unsafe {
            match self {
                ElementUsage::Position => {
                    gl::VertexPointer(count, ty, stride, ptr);
                    gl::EnableClientState(gl::VERTEX_ARRAY);
                }
                ElementUsage::Normal => {
                    gl::NormalPointer(ty, stride, ptr);
                    gl::EnableClientState(gl::NORMAL_ARRAY);
                }
                ElementUsage::Color => {
                    gl::ColorPointer(count, ty, stride, ptr);
                    gl::EnableClientState(gl::COLOR_ARRAY);
                }
                ElementUsage::Texture(index) => {
                    gl::ActiveTexture(gl::TEXTURE0 + *index);
                    gl::TexCoordPointer(count, ty, stride, ptr);
                    gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
                    gl::ActiveTexture(gl::TEXTURE0);
                }
                ElementUsage::Generic(index) => {
                    gl::EnableVertexAttribArray(*index);
                    gl::VertexAttribPointer(*index, count, ty, 0, stride, ptr);
                }
            }
        }
    }

    #[inline(always)]
    fn end(&self) {
        unsafe {
            match self {
                ElementUsage::Position => {
                    gl::DisableClientState(gl::VERTEX_ARRAY);
                }
                ElementUsage::Normal => {
                    gl::DisableClientState(gl::NORMAL_ARRAY);
                }
                ElementUsage::Color => {
                    gl::DisableClientState(gl::COLOR_ARRAY);
                }
                ElementUsage::Texture(index) => {
                    gl::ActiveTexture(gl::TEXTURE0 + *index);
                    gl::DisableClientState(gl::TEXTURE_COORD_ARRAY);
                    gl::ActiveTexture(gl::TEXTURE0);
                }
                ElementUsage::Generic(index) => {
                    gl::DisableVertexAttribArray(*index);
                }
            }
        }
    }
}

#[inline]
pub fn draw<V: Vertex, D: DrawTarget>(mode: DrawMode, vertices: &[V], target: &mut D, matrix: &Matrix4<f32>) {
    let _guard = target.bind();
    V::upload(vertices, mode, matrix)
}

#[inline]
pub fn draw_textured<'a, V: Vertex, D: DrawTarget>(mode: DrawMode, vertices: &[V], target: &mut D, texture: &TextureGuard<'a>, matrix: &Matrix4<f32>) {
    let _guard = target.bind();

    texture.context().texture2d.enable();
    V::upload(vertices, mode, matrix)
}

#[inline]
pub fn draw_gradient_rect<D: DrawTarget>(left: f32, top: f32, right: f32, bottom: f32, from: impl Into<Color> + Copy, to: impl Into<Color> + Copy, target: &mut D, matrix: &Matrix4<f32>) {
    let c = target.context();
    c.blend(Blend::default());
    c.shade_model(ShadeModel::Smooth);
    draw(DrawMode::Quads, &[
        PositionColor::new([right, top, 0.0], from),
        PositionColor::new([left, top, 0.0], from),
        PositionColor::new([left, bottom, 0.0], to),
        PositionColor::new([right, bottom, 0.0], to),
    ], target, matrix);
}

#[inline]
pub fn draw_textured_rect<'a, D: DrawTarget>(x: f32, y: f32, w: f32, h: f32, tx: f32, ty: f32, target: &mut D, texture: &TextureGuard<'a>, matrix: &Matrix4<f32>) {
    let scale_x = 1.0 / 256.0;
    let scale_y = 1.0 / 256.0;
    draw_textured(DrawMode::Quads, &[
        PositionTex::new([x, y + h, 0.0], [tx * scale_x, (ty + h) * scale_y]),
        PositionTex::new([x + w, y + h, 0.0], [(tx + w) * scale_x, (ty + h) * scale_y]),
        PositionTex::new([x + w, y, 0.0], [(tx + w) * scale_x, ty * scale_y]),
        PositionTex::new([x, y, 0.0], [tx * scale_x, ty * scale_y]),
    ], target, texture, matrix);
}

pub fn rgb(r: f32, g: f32, b: f32) -> [u8; 4] {
    let r = (r * 255.0) as u8;
    let g = (g * 255.0) as u8;
    let b = (b * 255.0) as u8;
    [r, g, b, 0xFF]
}

pub fn normal(x: f32, y: f32, z: f32) -> [i8; 3] {
    let x = (x * 127.0) as i8;
    let y = (y * 127.0) as i8;
    let z = (z * 127.0) as i8;
    [x, y, z]
}