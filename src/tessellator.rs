use crate::framebuffer::DrawTarget;
use crate::shader::{LinkedProgramId, ProgramError};
use crate::state::draw::DrawMode;
use crate::texture::TextureGuard;
use bytemuck::NoUninit;
use cgmath::{Matrix4, Vector1, Vector2, Vector3, Vector4};
use std::ffi::c_void;

use super::gl;
use super::gl::{GLenum, GLint};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

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

impl From<[f32; 4]> for Color {
    #[inline(always)]
    fn from(value: [f32; 4]) -> Self {
        let [r, g, b, a] = value;
        let r = (r.clamp(0.0, 1.0) * 255.0) as u8;
        let g = (g.clamp(0.0, 1.0) * 255.0) as u8;
        let b = (b.clamp(0.0, 1.0) * 255.0) as u8;
        let a = (a.clamp(0.0, 1.0) * 255.0) as u8;
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
    use super::Vertex;
    use crate::matrix::MatrixMode;
    use crate::shader::LinkedProgramId;
    use crate::state::draw::DrawMode;
    use crate::{gl, matrix};
    use cgmath::{Matrix, Matrix4};

    pub unsafe fn draw<V: Vertex>(mode: DrawMode, count: usize, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<V>>) {
        matrix::load(matrix, MatrixMode::ModelView);
        gl::UseProgram(program.map_or(0, |p| p.id()));
        gl::DrawArrays(mode as _, 0, count as i32);
    }

    unsafe impl<R, V> super::VertexSource<V> for R where R: AsRef<[V]>, V: Vertex {
        type Guard<'a> = () where Self: 'a;

        fn start(&self) -> *const V {
            self.as_ref().as_ptr().cast()
        }

        fn count(&self) -> usize {
            self.as_ref().len()
        }

        fn bind(&self) -> () {}
    }
}

pub trait Vertex: Sized + NoUninit {
    unsafe fn enable_client_state(start: *const Self);

    unsafe fn disable_client_state();

    #[inline(always)]
    fn draw<S: VertexSource<Self>>(source: S, mode: DrawMode, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<Self>>) {
        let count = source.count();
        if count > 0 {
            let _guard = source.bind();
            unsafe {
                Self::enable_client_state(source.start());
                formats::draw(mode, count, matrix, program);
                Self::disable_client_state();
            }
        }
    }

    fn bind_attributes(program: &LinkedProgramId<Self>) -> Result<(), ProgramError>;
}

pub unsafe trait VertexSource<V: Vertex> {
    type Guard<'a> where Self: 'a;

    fn start(&self) -> *const V;
    fn count(&self) -> usize;

    fn bind(&self) -> Self::Guard<'_>;
}

pub trait Element: Sized {
    #[inline(always)]
    fn size() -> usize {
        size_of::<Self>()
    }

    fn len() -> usize {
        1
    }

    fn gl() -> GLenum;
}

impl<E> Element for Vector4<E> where E: Element {
    #[inline(always)]
    fn len() -> usize {
        4
    }

    #[inline(always)]
    fn gl() -> GLenum {
        E::gl()
    }
}

impl<E> Element for Vector3<E> where E: Element {
    #[inline(always)]
    fn len() -> usize {
        3
    }

    #[inline(always)]
    fn gl() -> GLenum {
        E::gl()
    }
}

impl<E> Element for Vector2<E> where E: Element {
    #[inline(always)]
    fn len() -> usize {
        2
    }

    #[inline(always)]
    fn gl() -> GLenum {
        E::gl()
    }
}

impl<E> Element for Vector1<E> where E: Element {
    #[inline(always)]
    fn len() -> usize {
        1
    }

    #[inline(always)]
    fn gl() -> GLenum {
        E::gl()
    }
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
    pub unsafe fn begin<E: Element>(&self, stride: i32, ptr: *const c_void) {
        let count = E::len() as GLint;
        let ty = E::gl();
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
                gl::ClientActiveTexture(gl::TEXTURE0 + *index);
                gl::TexCoordPointer(count, ty, stride, ptr);
                gl::EnableClientState(gl::TEXTURE_COORD_ARRAY);
                gl::ClientActiveTexture(gl::TEXTURE0);
            }
            ElementUsage::Generic(index) => {
                gl::EnableVertexAttribArray(*index);
                gl::VertexAttribPointer(*index, count, ty, 0, stride, ptr);
            }
        }
    }

    #[inline(always)]
    pub unsafe fn end(&self) {
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
                gl::ClientActiveTexture(gl::TEXTURE0 + *index);
                gl::DisableClientState(gl::TEXTURE_COORD_ARRAY);
                gl::ClientActiveTexture(gl::TEXTURE0);
            }
            ElementUsage::Generic(index) => {
                gl::DisableVertexAttribArray(*index);
            }
        }
    }
}

#[inline]
pub fn draw<V: Vertex, S: VertexSource<V>, D: DrawTarget>(mode: DrawMode, vertices: S, target: &mut D, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<V>>) {
    let _guard = target.bind();
    V::draw(vertices, mode, matrix, program)
}

#[inline]
pub fn draw_textured<'a, V: Vertex, S: VertexSource<V>, D: DrawTarget>(mode: DrawMode, vertices: S, target: &mut D, texture: &TextureGuard<'a>, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<V>>) {
    let _guard = target.bind();

    texture.context().texture2d.enable();
    V::draw(vertices, mode, matrix, program)
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