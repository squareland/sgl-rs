use std::ops::{Add, Deref, Mul, Sub};
use std::rc::Rc;
use std::time::{Duration, Instant};
use cgmath::{Matrix4, Vector3};
use enumflags2::BitFlags;
use crate::gl;
use crate::texture::raw::{Texture, TextureGuard, TextureId};
use crate::state::{BufferBit, GraphicsContext};
use crate::state::draw::DrawMode;
use crate::tessellator::formats::PositionTex;

pub use crate::raw::buffer as raw;
pub use raw::*;
use crate::shader::LinkedProgramId;
use crate::texture::{InternalTextureFormat, MagFilter, MinFilter, UploadPixelFormat, TextureWrap};

pub struct Framebuffer {
    id: FramebufferId,
    texture: TextureId,
    depth: Option<RenderbufferId>,
    width: u32,
    height: u32,
    texture_width: u32,
    texture_height: u32,
    color: [f32; 4],
    context: GraphicsContext
}

impl Framebuffer {
    pub fn new<C: Into<GraphicsContext>>(width: u32, height: u32, color: [f32; 4], depth: bool, c: C) -> Result<Self, FramebufferStatus> {
        let context = c.into();
        let id = context.gen_framebuffer().expect("Could not allocate framebuffer");
        let texture = context.gen_texture().expect("Could not allocate texture");
        let depth = if depth { Some(context.gen_renderbuffer().expect("Could not allocate depth buffer")) } else { None };
        let bound_texture = texture.bind();
        bound_texture.wrap_s(TextureWrap::ClampToBorder); // FIXME: gl::CLAMP
        bound_texture.wrap_t(TextureWrap::ClampToBorder); // FIXME: gl::CLAMP
        bound_texture.min_filter(MinFilter::Nearest);
        bound_texture.mag_filter(MagFilter::Nearest);
        bound_texture.set_image::<u8>(0, InternalTextureFormat::RGBA8, width, height, UploadPixelFormat::RGBA, None);
        let bound_frame = id.bind();
        bound_frame.attach_texture(gl::COLOR_ATTACHMENT0, &bound_texture, 0);
        if let Some(ref d) = depth {
            let bound = d.bind();
            bound.storage(gl::DEPTH_COMPONENT24, width, height);
            bound_frame.renderbuffer(gl::DEPTH_ATTACHMENT, &bound);
        }

        let [r, g, b, a] = color;
        context.viewport(0, 0, width, height);
        context.clear_color(r, g, b, a);
        let mut bits: BitFlags<BufferBit> = BufferBit::Color.into();
        if depth.is_some() {
            context.clear_depth(1.0);
            bits |= BufferBit::Depth;
        }
        context.clear(bits);

        let status = bound_frame.check_status();

        drop(bound_frame);
        drop(bound_texture);

        match status {
            FramebufferStatus::Complete => Ok(Self {
                id,
                texture,
                depth,
                width,
                height,
                texture_width: width,
                texture_height: height,
                color,
                context
            }),
            other => Err(other)
        }
    }

    pub fn render<T: DrawTarget>(&self, w: u32, h: u32, disable_blend: bool, target: &mut T, matrix: Matrix4<f32>, program: Option<&LinkedProgramId<PositionTex>>) {
        let c = &self.context;

        c.color_mask(true, true, true, false);
        c.depth.disable();
        c.depth_mask(false);

        let m = matrix
            * cgmath::ortho(0.0, w as f32, h as f32, 0.0, 1000.0, 3000.0)
            * Matrix4::from_translation(-2000.0f32 * Vector3::unit_z());

        c.viewport(0, 0, w, h);
        c.texture2d.enable();
        c.lighting.disable();
        c.alpha.disable();

        if disable_blend {
            c.blend.disable();
            c.color_material.enable();
        }

        let x = 0.0;
        let y = 0.0;
        let w = w as f32;
        let h = h as f32;
        let tw = self.width as f32 / self.texture_width as f32;
        let th = self.height as f32 / self.texture_height as f32;

        super::tessellator::draw_textured(DrawMode::Quads, &[
            PositionTex::new([x, y + h, 0.0], [0.0, 0.0]),
            PositionTex::new([x + w, y + h, 0.0], [tw, 0.0]),
            PositionTex::new([x + w, y, 0.0], [tw, th]),
            PositionTex::new([x, y, 0.0], [0.0, th]),
        ], target, &self.texture.bind(), &m, program);

        c.depth_mask(true);
        c.color_mask(true, true, true, true);
    }

    pub fn clear(&mut self) {
        let _bound = self.id.bind();
        let [r, g, b, a] = self.color;
        self.context.clear_color(r, g, b, a);
        let mut bits: BitFlags<BufferBit> = BufferBit::Color.into();
        if self.depth.is_some() {
            self.context.clear_depth(1.0);
            bits |= BufferBit::Depth;
        }
        self.context.clear(bits);
    }
}

impl<'a> Texture<'a> for &'a Framebuffer {
    #[inline(always)]
    fn bind(self) -> Rc<TextureGuard<'a>> {
        self.texture.bind()
    }
}

pub struct Frame {
    pub context: GraphicsContext,
    pub start: Instant,
    elapsed: f32,
    display_width: u32,
    display_height: u32
}

impl Frame {
    pub unsafe fn new<C: Into<GraphicsContext>>(c: C, elapsed: f32, display_width: u32, display_height: u32) -> Self {
        Self { context: c.into(), start: Instant::now(), elapsed, display_width, display_height }
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    pub fn frame_duration(&self) -> Duration {
        Instant::now().duration_since(self.start)
    }

    pub fn lerp<V>(&self, from: V, to: V) -> V where V: Mul<f32, Output = V> + Add<Output=V> + Sub<Output=V> + Copy {
        from + (to - from) * self.elapsed
    }

    pub fn display_width(&self) -> u32 {
        self.display_width
    }

    pub fn display_height(&self) -> u32 {
        self.display_height
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.display_width as f32 / self.display_height as f32
    }

    pub fn reset_viewport(&self) {
        self.viewport(0, 0, self.display_width, self.display_height);
    }
}

impl Deref for Frame {
    type Target = GraphicsContext;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl DrawTarget for Frame {
    type Guard<'a> = () where Self: 'a;

    #[inline(always)]
    fn bind(&self) -> Self::Guard<'_> {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    #[inline(always)]
    fn context(&self) -> &GraphicsContext {
        &self.context
    }
}

impl DrawTarget for Framebuffer {
    type Guard<'a> = FramebufferGuard<'a> where Self: 'a;

    #[inline(always)]
    fn bind(&self) -> Self::Guard<'_> {
        self.id.bind()
    }

    #[inline(always)]
    fn context(&self) -> &GraphicsContext {
        &self.context
    }
}