use std::ffi::c_void;
use bytemuck::NoUninit;
use cgmath::{Matrix4, Vector1, Vector2, Vector3, Vector4};
use crate::framebuffer::DrawTarget;
use crate::shader::{LinkedProgramId, ProgramError};
use crate::state::blend::Blend;
use crate::state::draw::DrawMode;
use crate::state::shade::ShadeModel;
use crate::texture::TextureGuard;
use self::formats::{PositionColor, PositionTex};

use super::gl;
use super::gl::GLenum;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
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

impl From<[f32; 4]> for Color {
    #[inline(always)]
    fn from(value: [f32; 4]) -> Self {
        let [r, g, b, a] = value;
        Self((r / 255.0) as u8, (g / 255.0) as u8, (b / 255.0) as u8, (a / 255.0) as u8)
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
    use crate::gl;
    use crate::state::draw::DrawMode;
    use crate::shader::{LinkedProgramId};
    use cgmath::{Matrix4, Matrix, Vector3};

    #[macro_export]
    macro_rules! vertex {
        ($name:ident($(#[$usage:expr] $field:ident : $ty:ty),*)) => {
            #[repr(C)]
            #[derive(Copy, Clone, Debug)]
            pub struct $name {
                $(pub $field: $ty),+
            }

            unsafe impl bytemuck::NoUninit for $name {}

            impl $crate::tessellator::Vertex for $name {
                unsafe fn enable_client_state(start: *const $name) {
                    let size = std::mem::size_of::<Self>();
                    $(
                        let offset = field_offset::offset_of!(Self => $field);
                        let len = <$ty as $crate::tessellator::Element>::len();
                        let gl = <$ty as $crate::tessellator::Element>::gl();
                        let usage = $usage;
                        usage.begin(len as _, gl, size as i32, offset.apply_ptr(start).cast());
                    )*
                }

                unsafe fn disable_client_state() {
                    $(
                        $usage.end();
                    )*
                }

                #[inline(always)]
                fn draw<S: $crate::tessellator::VertexSource<$name>>(source: S, mode: DrawMode, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<Self>>) {
                    let count = source.count();
                    if count > 0 {
                        let _guard = source.bind();
                        unsafe {
                            Self::enable_client_state(source.start());
                            crate::matrix::load(matrix, $crate::matrix::MatrixMode::ModelView);
                            gl::UseProgram(program.map_or(0, |p| p.id()));
                            gl::DrawArrays(mode as _, 0, count as i32);
                            Self::disable_client_state();
                        }
                    }
                }

                #[inline(always)]
                fn bind_attributes(program: &LinkedProgramId<Self>) -> Result<(), $crate::shader::ProgramError> {
                    let mut id = 0;
                    $(
                        unsafe {
                            const CNAME: &[u8] = concat!(stringify!($field), "\u{0000}").as_bytes();
                            let name = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(CNAME) };
                            let attribute = program.attribute::<$ty>(name)
                                .ok_or($crate::shader::ProgramError::AttributeMissing(stringify!($field)))?;
                            attribute.bind(id);
                            id += 1;
                        }
                    )*
                    Ok(())
                }
            }

            impl $name {
                pub fn new($($field: impl Into<$ty>),*) -> Self {
                    Self {
                        $($field: $field.into()),*
                    }
                }
            }

            unsafe impl<R> $crate::tessellator::VertexSource<$name> for R where R: AsRef<[$name]> {
                type Guard<'a> = () where Self: 'a;

                fn start(&self) -> *const $name {
                    self.as_ref().as_ptr().cast()
                }

                fn count(&self) -> usize {
                    self.as_ref().len()
                }

                fn bind(&self) -> () {
                }
            }
        };
    }

    vertex!(Position(
        #[ElementUsage::Position]
        pos: Vector3<f32>
    ));
    vertex!(PositionColor(
        #[ElementUsage::Position]
        pos: Vector3<f32>,
        #[ElementUsage::Color]
        color: Color
    ));
    vertex!(PositionTex(
        #[ElementUsage::Position]
        pos: Vector3<f32>,
        #[ElementUsage::Texture(0)]
        uv: [f32; 2]
    ));
    vertex!(PositionTexNormal(
        #[ElementUsage::Position]
        pos: Vector3<f32>,
        #[ElementUsage::Texture(0)]
        uv: [f32; 2],
        #[ElementUsage::Normal]
        normal: Vector3<i8>
    ));
    vertex!(PositionTexNormalColor(
        #[ElementUsage::Position]
        pos: Vector3<f32>,
        #[ElementUsage::Texture(0)]
        uv: [f32; 2],
        #[ElementUsage::Normal]
        normal: Vector3<i8>,
        #[ElementUsage::Color]
        color: Color
    ));
    vertex!(PositionNormal(
        #[ElementUsage::Position]
        pos: Vector3<f32>,
        #[ElementUsage::Normal]
        normal: Vector3<i8>
    ));
    vertex!(PositionTexColor(
        #[ElementUsage::Position]
        pos: Vector3<f32>,
        #[ElementUsage::Texture(0)]
        uv: [f32; 2],
        #[ElementUsage::Color]
        color: Color
    ));
}

pub trait Vertex: Sized + NoUninit {
    unsafe fn enable_client_state(start: *const Self);

    unsafe fn disable_client_state();

    fn draw<S: VertexSource<Self>>(source: S, mode: DrawMode, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<Self>>);

    fn bind_attributes(program: &LinkedProgramId<Self>) -> Result<(), ProgramError>;
}

pub unsafe trait VertexSource<V: Vertex> {
    type Guard<'a> where Self: 'a;

    fn start(&self) -> *const V;
    fn count(&self) -> usize;

    fn bind(&self) -> Self::Guard<'_>;
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

#[inline]
pub fn draw_gradient_rect<D: DrawTarget>(left: f32, top: f32, right: f32, bottom: f32, from: impl Into<Color> + Copy, to: impl Into<Color> + Copy, target: &mut D, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<PositionColor>>) {
    let c = target.context();
    c.texture2d.disable();
    c.alpha.disable();
    c.blend(Blend::default());
    c.shade_model(ShadeModel::Smooth);
    draw(DrawMode::Quads, &[
        PositionColor::new([right, top, 0.0], from),
        PositionColor::new([left, top, 0.0], from),
        PositionColor::new([left, bottom, 0.0], to),
        PositionColor::new([right, bottom, 0.0], to),
    ], target, matrix, program);
    let c = target.context();
    c.shade_model(ShadeModel::Flat);
    c.blend.disable();
    c.alpha.enable();
    c.texture2d.enable();
}

#[inline]
pub fn draw_textured_rect<'a, D: DrawTarget>(x: f32, y: f32, w: f32, h: f32, tx: f32, ty: f32, target: &mut D, texture: &TextureGuard<'a>, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<PositionTex>>) {
    let scale_x = 1.0 / 256.0;
    let scale_y = 1.0 / 256.0;
    draw_textured(DrawMode::Quads, &[
        PositionTex::new([x, y + h, 0.0], [tx * scale_x, (ty + h) * scale_y]),
        PositionTex::new([x + w, y + h, 0.0], [(tx + w) * scale_x, (ty + h) * scale_y]),
        PositionTex::new([x + w, y, 0.0], [(tx + w) * scale_x, ty * scale_y]),
        PositionTex::new([x, y, 0.0], [tx * scale_x, ty * scale_y]),
    ], target, texture, matrix, program);
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