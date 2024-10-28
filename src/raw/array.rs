use std::num::NonZeroU32;
use crate::gl;
use crate::state::GraphicsContext;

#[repr(transparent)]
pub struct VertexArrayId(pub NonZeroU32, pub(crate) GraphicsContext);

impl VertexArrayId {
    #[inline(always)]
    pub fn bind(&self) -> VertexArrayGuard {
        unsafe {
            gl::BindVertexArray(self.0.get());
        }
        VertexArrayGuard {
            array: self
        }
    }
}

pub struct VertexArrayGuard<'array> {
    array: &'array VertexArrayId
}

impl<'array> Drop for VertexArrayGuard<'array> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            gl::BindVertexArray(0)
        }
    }
}