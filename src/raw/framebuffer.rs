use std::num::NonZeroU32;
use crate::gl;
use crate::debug::gl_enum;
use crate::state::GraphicsContext;
use super::texture::TextureGuard;
use super::types::GLenum;

pub trait DrawTarget {
    type Guard<'a> where Self: 'a;

    fn bind(&self) -> Self::Guard<'_>;

    fn context(&self) -> &GraphicsContext;
}

#[repr(transparent)]
pub struct FramebufferId(pub NonZeroU32, pub(crate) GraphicsContext);
#[repr(transparent)]
pub struct RenderbufferId(pub NonZeroU32, pub(crate) GraphicsContext);

impl FramebufferId {
    #[inline(always)]
    pub fn bind(&self) -> FramebufferGuard {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.0.get());
        }
        FramebufferGuard {
            _buffer: self
        }
    }
}

impl RenderbufferId {
    #[inline(always)]
    pub fn bind(&self) -> RenderbufferGuard {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.0.get());
        }
        RenderbufferGuard {
            buffer: self
        }
    }
}

impl Drop for FramebufferId {
    #[inline(always)]
    fn drop(&mut self) {
        let id = self.0.get();
        unsafe {
            gl::DeleteFramebuffers(1, &id)
        }
    }
}

impl Drop for RenderbufferId {
    #[inline(always)]
    fn drop(&mut self) {
        let id = self.0.get();
        unsafe {
            gl::DeleteRenderbuffers(1, &id)
        }
    }
}

pub struct FramebufferGuard<'buffer> {
    _buffer: &'buffer FramebufferId
}

pub struct RenderbufferGuard<'buffer> {
    buffer: &'buffer RenderbufferId
}

impl<'buffer> FramebufferGuard<'buffer> {
    #[inline(always)]
    pub fn check_status(&self) -> FramebufferStatus {
        unsafe {
            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            FramebufferStatus::from(status)
        }
    }

    #[inline(always)]
    pub fn attach_texture(&self, attachment: GLenum, texture: &TextureGuard, level: i32) {
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment, gl::TEXTURE_2D, texture.id().get(), level)
        }
    }

    #[inline(always)]
    pub fn renderbuffer(&self, attachment: GLenum, render: &RenderbufferGuard) {
        unsafe {
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, attachment, gl::RENDERBUFFER, render.buffer.0.get());
        }
    }
}

impl<'buffer> RenderbufferGuard<'buffer> {
    #[inline(always)]
    pub fn storage(&self, format: GLenum, width: u32, height: u32) {
        unsafe {
            gl::RenderbufferStorage(gl::RENDERBUFFER, format, width as _, height as _);
        }
    }
}

impl<'buffer> Drop for FramebufferGuard<'buffer> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0)
        }
    }
}

impl<'buffer> Drop for RenderbufferGuard<'buffer> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0)
        }
    }
}

gl_enum! {
    #[derive(Debug)]
    pub enum FramebufferStatus(u32) {
        Complete                    = gl::FRAMEBUFFER_COMPLETE,
        Undefined                   = gl::FRAMEBUFFER_UNDEFINED,
        Unsupported                 = gl::FRAMEBUFFER_UNSUPPORTED,
        IncompleteAttachment        = gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT,
        IncompleteMissingAttachment = gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,
        IncompleteDrawBuffer        = gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER,
        IncompleteReadBuffer        = gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER,
        IncompleteMultisample       = gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
        IncompleteLayerTargets      = gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS
    }
}