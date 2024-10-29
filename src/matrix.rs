use cgmath::{Matrix, Matrix4};
use super::gl;

#[inline(always)]
pub fn load(matrix: &Matrix4<f32>, mode: MatrixMode) {
    unsafe {
        gl::MatrixMode(mode as _);
        gl::LoadMatrixf(matrix.as_ptr());
    }
}

#[repr(u32)]
pub enum MatrixMode {
    Projection = gl::PROJECTION,
    ModelView = gl::MODELVIEW,
    Texture = gl::TEXTURE
}