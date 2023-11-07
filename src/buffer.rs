use std::marker::PhantomData;
use cgmath::Matrix4;
use crate::framebuffer::{BufferGuard, BufferId, BufferKind, UploadMode};
use crate::shader::LinkedProgramId;
use crate::state::draw::DrawMode;
use crate::state::GraphicsContext;
use crate::tessellator::{Vertex, VertexSource};

pub struct VertexBuffer<V> {
    id: BufferId<{ BufferKind::Array }>,
    vertices: usize,
    context: GraphicsContext,
    _phantom: PhantomData<*const V>
}

impl<V: Vertex> VertexBuffer<V> {
    pub fn new<C: Into<GraphicsContext>>(c: C) -> Self {
        let context = c.into();
        let id = context.gen_buffer().expect("Could not allocate framebuffer");

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