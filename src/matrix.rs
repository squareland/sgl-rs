use cgmath::{Matrix, Matrix4};
use super::gl;

#[inline(always)]
pub fn mul(matrix: &Matrix4<f32>, block: impl FnOnce()) {
    unsafe {
        gl::MatrixMode(gl::MODELVIEW);
        gl::LoadMatrixf(matrix.as_ptr());
        block();
    }
}