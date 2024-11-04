use crate::framebuffer::{BufferAccess, BufferGuard, BufferId, BufferKind, MappedBuffer, UploadMode};
use crate::raw::fence::Sync;
use crate::shader::LinkedProgramId;
use crate::state::draw::DrawMode;
use crate::state::pixel::PixelFormat;
use crate::state::GraphicsContext;
use crate::tessellator::{Vertex, VertexSource};
use crate::texture::{Pixel, TextureGuard};
use cgmath::Matrix4;
use std::marker::PhantomData;

pub struct VertexBuffer<V> {
    id: BufferId<{ BufferKind::Array }>,
    vertices: usize,
    context: GraphicsContext,
    _phantom: PhantomData<*const V>
}

impl<V: Vertex> VertexBuffer<V> {
    pub fn new<C: Into<GraphicsContext>>(c: C) -> Self {
        let context = c.into();
        let id = context.gen_buffer().expect("Could not allocate vertex buffer");

        Self {
            id,
            vertices: 0,
            context,
            _phantom: PhantomData
        }
    }

    #[inline(always)]
    pub fn upload(&mut self, vertices: &[V], mode: UploadMode) {
        let data = bytemuck::cast_slice(vertices);
        self.id.bind().upload(data, mode);
        self.vertices = vertices.len();
    }

    #[inline(always)]
    pub fn sub_upload(&mut self, offset: usize, vertices: &[V]) {
        assert!(offset + vertices.len() <= self.vertices, "Sub upload outside of allocated buffer area");
        let data = bytemuck::cast_slice(vertices);
        self.id.bind().sub_upload(offset, data);
    }

    #[inline(always)]
    pub fn draw(&self, mode: DrawMode, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<V>>) {
        let _bound = self.id.bind();
        V::draw(self, mode, matrix, program);
    }

    #[inline(always)]
    pub fn draw_textured<'a>(&self, mode: DrawMode, matrix: &Matrix4<f32>, texture: &TextureGuard<'a>, program: Option<&LinkedProgramId<V>>) {
        texture.context().texture2d.enable();
        self.draw(mode, matrix, program);
    }
}

unsafe impl<V: Vertex> VertexSource<V> for &VertexBuffer<V> {
    type Guard<'buffer> = BufferGuard<'buffer, { BufferKind::Array }> where Self: 'buffer;

    fn start(&self) -> *const V {
        std::ptr::null()
    }

    fn count(&self) -> usize {
        self.vertices
    }

    fn bind(&self) -> Self::Guard<'_> {
        self.id.bind()
    }
}

pub struct PixelPackBuffer<P> {
    id: BufferId<{ BufferKind::PixelPack }>,
    size: usize,
    context: GraphicsContext,
    _phantom: PhantomData<*const P>
}

impl<P> PixelPackBuffer<P> {
    pub fn new<C: Into<GraphicsContext>>(c: C) -> Self {
        let context = c.into();
        let id = context.gen_buffer().expect("Could not allocate pixel pack buffer");
        
        Self {
            id,
            size: 0,
            context,
            _phantom: PhantomData,
        }
    }
}

impl<P> PixelPackBuffer<P> where P: Pixel {
    #[inline(always)]
    pub fn allocate(&mut self, pixels: usize, mode: UploadMode) {
        self.size = P::size() * pixels;
        self.id.bind().allocate(self.size, mode);
    }
    
    #[inline(always)]
    pub fn download(&self, x: u32, y: u32, width: u32, height: u32, format: PixelFormat) -> Sync {
        assert!(self.size > 0, "cannot download to unallocated buffer");
        unsafe {
            self.id.bind().read_pixels::<P>(x, y, width, height, format)
        }
    }
}

pub struct PixelUnpackBuffer<P> {
    id: BufferId<{ BufferKind::PixelUnpack }>,
    size: usize,
    context: GraphicsContext,
    _phantom: PhantomData<*const P>
}

impl<P> PixelUnpackBuffer<P> {
    pub fn new<C: Into<GraphicsContext>>(c: C) -> Self {
        let context = c.into();
        let id = context.gen_buffer().expect("Could not allocate pixel unpack buffer");
        
        Self {
            id,
            size: 0,
            context,
            _phantom: PhantomData,
        }
    }
}

impl<P> PixelUnpackBuffer<P> where P: Pixel {
    #[inline(always)]
    pub fn upload(&mut self, pixels: &[P], mode: UploadMode) {
        self.size = P::size() * pixels.len();
        let data = bytemuck::cast_slice(pixels);
        self.id.bind().upload(data, mode);
    }
    
    #[inline(always)]
    pub fn allocate(&mut self, pixels: usize, mode: UploadMode) {
        self.size = P::size() * pixels;
        self.id.bind().allocate(self.size, mode);
    }
    
    #[inline(always)]
    pub fn mapped(&mut self, access: BufferAccess) -> MappedBuffer<{ BufferKind::PixelUnpack }, P> {
        assert!(self.size > 0, "cannot map unallocated buffer");
        unsafe {
            self.id.bind().mapped(self.size, access)
        }
    }
}