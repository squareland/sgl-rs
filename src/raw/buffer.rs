use std::ffi::c_void;
use std::marker::{ConstParamTy, PhantomData};
use std::mem::ManuallyDrop;
use std::num::NonZeroU32;
use std::ops::{Deref, DerefMut};
use crate::gl;
use crate::debug::gl_enum;
use crate::raw::fence::{Sync, SyncCondition};
use crate::state::GraphicsContext;
use crate::state::pixel::PixelFormat;
use crate::texture::Pixel;
use super::texture::TextureGuard;
use super::GLenum;

pub trait DrawTarget {
    type Guard<'a> where Self: 'a;

    fn bind(&self) -> Self::Guard<'_>;

    fn context(&self) -> &GraphicsContext;
}

#[repr(transparent)]
pub struct FramebufferId(pub NonZeroU32, pub(crate) GraphicsContext);
#[repr(transparent)]
pub struct RenderbufferId(pub NonZeroU32, pub(crate) GraphicsContext);
#[repr(transparent)]
pub struct BufferId<const K: BufferKind>(pub NonZeroU32, pub(crate) GraphicsContext);

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, ConstParamTy)]
pub enum BufferKind {
    Array = gl::ARRAY_BUFFER,
    AtomicCounter = gl::ATOMIC_COUNTER_BUFFER,
    CopyRead = gl::COPY_READ_BUFFER,
    CopyWrite = gl::COPY_WRITE_BUFFER,
    DispatchIndirect = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirect = gl::DRAW_INDIRECT_BUFFER,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER,
    PixelPack = gl::PIXEL_PACK_BUFFER,
    PixelUnpack = gl::PIXEL_UNPACK_BUFFER,
    Query = gl::QUERY_BUFFER,
    ShaderStorage = gl::SHADER_STORAGE_BUFFER,
    Texture = gl::TEXTURE_BUFFER,
    TransformFeedback = gl::TRANSFORM_FEEDBACK,
    Uniform = gl::UNIFORM_BUFFER,
}

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

impl<const K: BufferKind> BufferId<K> {
    #[inline(always)]
    pub fn bind(&self) -> BufferGuard<K> {
        unsafe {
            gl::BindBuffer(K as _, self.0.get());
        }
        BufferGuard {
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

impl<const K: BufferKind> Drop for BufferId<K> {
    #[inline(always)]
    fn drop(&mut self) {
        let id = self.0.get();
        unsafe {
            gl::DeleteBuffers(1, &id)
        }
    }
}

pub struct FramebufferGuard<'buffer> {
    _buffer: &'buffer FramebufferId
}

pub struct RenderbufferGuard<'buffer> {
    buffer: &'buffer RenderbufferId
}

pub struct BufferGuard<'buffer, const K: BufferKind> {
    buffer: &'buffer BufferId<K>
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
    pub fn attach_color_texture(&self, i: u32, texture: &TextureGuard) {
        unsafe {
            let mut max = 0;
            gl::GetIntegerv(gl::MAX_COLOR_ATTACHMENTS, &mut max);
            assert!(i < max as u32, "color attachment id is larger than maximum supported: {} > {}", i, max - 1);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0 + i, gl::TEXTURE_2D, texture.id().get(), 0)
        }
    }

    #[inline(always)]
    pub fn attach_color_buffer(&self, i: u32, render: &RenderbufferGuard) {
        unsafe {
            let mut max = 0;
            gl::GetIntegerv(gl::MAX_COLOR_ATTACHMENTS, &mut max);
            assert!(i < max as u32, "color attachment id is larger than maximum supported: {} > {}", i, max - 1);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0 + i, gl::RENDERBUFFER, render.buffer.0.get());
        }
    }

    #[inline(always)]
    pub fn attach_depth_buffer(&self, render: &RenderbufferGuard) {
        unsafe {
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, render.buffer.0.get());
        }
    }

    #[inline(always)]
    pub fn attach_depth_texture(&self, texture: &TextureGuard) {
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, texture.id().get(), 0);
        }
    }

    #[inline(always)]
    pub fn attach_stencil_buffer(&self, render: &RenderbufferGuard) {
        unsafe {
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::STENCIL_ATTACHMENT, gl::RENDERBUFFER, render.buffer.0.get());
        }
    }

    #[inline(always)]
    pub fn attach_stencil_texture(&self, texture: &TextureGuard) {
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::STENCIL_ATTACHMENT, gl::TEXTURE_2D, texture.id().get(), 0);
        }
    }

    #[inline(always)]
    pub fn attach_depth_stencil_buffer(&self, render: &RenderbufferGuard) {
        unsafe {
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, render.buffer.0.get());
        }
    }

    #[inline(always)]
    pub fn attach_depth_stencil_texture(&self, texture: &TextureGuard) {
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, texture.id().get(), 0);
        }
    }
}

impl<'buffer, const K: BufferKind> BufferGuard<'buffer, K> {
    #[inline(always)]
    pub fn allocate(&self, bytes: usize, mode: UploadMode) {
        unsafe {
            gl::BufferData(K as _, bytes as _, std::ptr::null(), mode as _);
        }
    }
    
    #[inline(always)]
    pub fn upload(&self, data: &[u8], mode: UploadMode) {
        unsafe {
            gl::BufferData(K as _, data.len() as _, data.as_ptr().cast(), mode as _);
        }
    }

    #[inline(always)]
    pub fn sub_upload(&self, offset: usize, data: &[u8]) {
        unsafe {
            gl::BufferSubData(K as _, offset as _, data.len() as _, data.as_ptr().cast());
        }
    }

    #[inline(always)]
    pub unsafe fn mapped<V>(self, size: usize, access: BufferAccess) -> MappedBuffer<'buffer, K, V> {
        let ptr = unsafe {
            gl::MapBuffer(K as _, access as _)
        };
        assert_eq!(ptr.align_offset(align_of::<V>()), 0, "unaligned mapped buffer");
        assert!(!ptr.is_null(), "ptr is null");
        MappedBuffer {
            buffer: self,
            ptr,
            size,
            _phantom: PhantomData,
        }
    }
}

impl<'buffer> BufferGuard<'buffer, { BufferKind::PixelPack }> {
    pub unsafe fn read_pixels<P>(&self, x: u32, y: u32, width: u32, height: u32, format: PixelFormat) -> Sync where P: Pixel {
        unsafe {
            gl::ReadPixels(x as _, y as _, width as _, height as _, format as _, P::gl_type() as _, std::ptr::null_mut());
        }
        self.buffer.1.fence(SyncCondition::GpuCommandsComplete)
    }
}

pub struct MappedBuffer<'buffer, const K: BufferKind, V> {
    buffer: BufferGuard<'buffer, K>,
    ptr: *mut c_void,
    size: usize,
    _phantom: PhantomData<*mut V>,
}

impl<'buffer, const K: BufferKind, V> MappedBuffer<'buffer, K, V> {
    pub fn unmap(self) -> BufferGuard<'buffer, K> {
        let this = ManuallyDrop::new(self);
        unsafe {
            gl::UnmapBuffer(K as _);
        }
        BufferGuard {
            buffer: this.buffer.buffer
        }
    }
}

impl<'buffer, const K: BufferKind, V> Drop for MappedBuffer<'buffer, K, V> {
    fn drop(&mut self) {
        unsafe {
            gl::UnmapBuffer(K as _);
        }
    }
}

impl<'buffer, const K: BufferKind, V> Deref for MappedBuffer<'buffer, K, V> {
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr.cast(), self.size)
        }
    }
}

impl<'buffer, const K: BufferKind, V> DerefMut for MappedBuffer<'buffer, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.cast(), self.size)
        }
    }
}

#[repr(u32)]
pub enum UploadMode {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

#[repr(u32)]
pub enum BufferAccess {
    ReadOnly = gl::READ_ONLY,
    WriteOnly = gl::WRITE_ONLY,
    ReadWrite = gl::READ_WRITE,
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

impl<'buffer, const K: BufferKind> Drop for BufferGuard<'buffer, K> {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            gl::BindBuffer(K as _, 0)
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