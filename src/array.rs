use crate::framebuffer::{BufferId, BufferKind, UploadMode};
use crate::raw::array::VertexArrayId;
use crate::shader::LinkedProgramId;
use crate::state::draw::DrawMode;
use crate::state::GraphicsContext;
use crate::tessellator::Vertex;
use crate::texture::TextureGuard;
use cgmath::Matrix4;
use std::marker::PhantomData;

pub struct VertexArray<V> {
    id: VertexArrayId,
    buffer_id: BufferId<{ BufferKind::Array }>,
    vertices: usize,
    context: GraphicsContext,
    _phantom: PhantomData<*const V>
}

impl<V: Vertex> VertexArray<V> {
    pub fn new<C: Into<GraphicsContext>>(c: C) -> Self {
        let context = c.into();
        let id = context.gen_vertex_array().expect("Could not allocate vertex array");
        let buffer_id = context.gen_buffer().expect("Could not allocate vertex buffer");

        {
            let _bound = id.bind();
            let _bound_buffer = buffer_id.bind();
            unsafe {
                V::enable_client_state(std::ptr::null());
            }
        }

        Self {
            id,
            buffer_id,
            vertices: 0,
            context,
            _phantom: PhantomData
        }
    }

    #[inline(always)]
    pub fn upload(&mut self, vertices: &[V], mode: UploadMode) {
        let data = bytemuck::cast_slice(vertices);
        self.buffer_id.bind().upload(data, mode);
        self.vertices = vertices.len();
    }

    #[inline(always)]
    pub fn sub_upload(&mut self, offset: usize, vertices: &[V]) {
        assert!(offset + vertices.len() <= self.vertices, "Sub upload outside of allocated buffer area");
        let data = bytemuck::cast_slice(vertices);
        self.buffer_id.bind().sub_upload(offset, data);
    }

    #[inline(always)]
    pub fn draw(&self, mode: DrawMode, matrix: &Matrix4<f32>, program: Option<&LinkedProgramId<V>>) {
        let _bound = self.id.bind();
        let count = self.vertices;
        if count > 0 {
            unsafe {
                crate::tessellator::formats::draw(mode, count, matrix, program);
            }
        }
    }

    #[inline(always)]
    pub fn draw_textured<'a>(&self, mode: DrawMode, matrix: &Matrix4<f32>, texture: &TextureGuard<'a>, program: Option<&LinkedProgramId<V>>) {
        self.draw(mode, matrix, program);
    }
}