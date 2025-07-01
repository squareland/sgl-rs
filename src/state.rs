use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::transmute;
use std::num::NonZeroU32;
use cgmath::{Array, Vector3, Vector4};
use enumflags2::{bitflags, BitFlags};
use crate::gl;
use crate::debug::GlError;
use crate::framebuffer::{BufferId, BufferKind, FramebufferId, RenderbufferId};
use crate::query::QueryId;
use crate::raw::array::VertexArrayId;
use crate::raw::display::DisplayListIter;
use crate::state::alpha::AlphaFunc;
use crate::state::blend::{Blend, DstFactor, SrcFactor};
use crate::state::color::{ColorMaterial, ColorMode};
use crate::state::depth::DepthFunc;
use crate::state::face::Face;
use crate::state::fog::{Fog, FogMode};
use crate::state::light::LightModel;
use crate::state::pixel::PixelFormat;
use crate::state::shade::ShadeModel;
use crate::state::stencil::{StencilOp, StencilFunc, Stencil};
use crate::texture::Pixel;
use crate::texture::raw::TextureId;
use super::raw::GLenum;

#[derive(PartialEq, Copy)]
pub struct GraphicsContext {
    pub alpha: BooleanState<{ gl::ALPHA_TEST }>,
    pub auto_normal: BooleanState<{ gl::AUTO_NORMAL }>,
    pub blend: BooleanState<{ gl::BLEND }>,
    pub cull_face: BooleanState<{ gl::CULL_FACE }>,
    pub depth: BooleanState<{ gl::DEPTH_TEST }>,
    pub dither: BooleanState<{ gl::DITHER }>,
    pub fog: BooleanState<{ gl::FOG }>,
    pub texture2d: BooleanState<{ gl::TEXTURE_2D }>,
    pub lighting: BooleanState<{ gl::LIGHTING }>,
    pub color_material: BooleanState<{ gl::COLOR_MATERIAL }>,
    pub debug_output: BooleanState<{ gl::DEBUG_OUTPUT }>,
    pub rescale_normal: BooleanState<{ gl::RESCALE_NORMAL }>,
    pub stencil: BooleanState<{ gl::STENCIL_TEST }>,
    pub scissor: BooleanState<{ gl::SCISSOR_TEST }>,
    _private: PhantomData<*const ()>
}

#[derive(Copy, Clone)]
pub struct DrawParams {
    pub alpha: Option<AlphaFunc>,
    pub auto_normal: bool,
    pub blend: Option<Blend>,
    pub cull_face: Option<Face>,
    pub depth: Option<DepthFunc>,
    pub color_mask: (bool, bool, bool, bool),
    pub depth_mask: bool,
    pub dither: bool,
    pub fog: Option<Fog>,
    pub lighting: bool,
    pub rescale_normal: bool,
    pub color_material: Option<ColorMaterial>,
    pub stencil: Option<Stencil>,
    pub shade_model: ShadeModel,
    pub line_width: f32,
    pub color: Vector4<f32>,
    pub viewport: Viewport,
}

#[derive(Copy, Clone)]
pub struct Viewport {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl DrawParams {
    pub fn default(viewport: Viewport) -> Self {
        Self {
            alpha: Some(AlphaFunc::Always),
            auto_normal: false,
            blend: Some(Blend::default()),
            cull_face: Some(Face::Back),
            depth: Some(DepthFunc::Less),
            color_mask: (true, true, true, true),
            depth_mask: true,
            dither: false,
            fog: None,
            lighting: false,
            rescale_normal: false,
            color_material: None,
            stencil: None,
            shade_model: ShadeModel::Flat,
            line_width: 1.0,
            color: Vector4::from([1.0, 1.0, 1.0, 1.0]),
            viewport
        }
    }

    pub fn apply(&self, context: &mut GraphicsContext) {
        context.viewport(self.viewport.x, self.viewport.y, self.viewport.w, self.viewport.h);
        if let Some(alpha) = self.alpha {
            context.alpha.enable();
            context.alpha_func(alpha);
        } else {
            context.alpha.disable();
        }
        if self.auto_normal {
            context.auto_normal.enable();
        } else {
            context.auto_normal.disable();
        }
        if let Some(blend) = self.blend {
            context.blend.enable();
            context.blend_func(blend.src_color, blend.dst_color, blend.src_alpha, blend.dst_alpha);
        } else {
            context.blend.disable();
        }
        if let Some(cull_face) = self.cull_face {
            context.cull_face.enable();
            context.cull_face(cull_face);
        } else {
            context.cull_face.disable();
        }
        if let Some(depth) = self.depth {
            context.depth.enable();
            context.depth_func(depth);
        } else {
            context.depth.disable();
        }
        let (r, g, b, a) = self.color_mask;
        context.color_mask(r, g, b, a);
        context.depth_mask(self.depth_mask);
        if self.dither {
            context.dither.enable();
        } else {
            context.dither.disable();
        }
        if let Some(fog) = self.fog {
            context.fog_color(fog.color);
            context.reset_color();
            context.fog_mode(fog.mode);
            // context.fog_density(fog.density);
            context.fog_start(fog.start);
            context.fog_end(fog.end);
            if let Some(nv) = fog.nv {
                context.fog_distance_nv(nv.distance);
                context.fog_eye_plane_nv(nv.eye_plane);
            }
            context.color_material(Face::Front, ColorMode::Ambient);
            context.color(fog.color.x, fog.color.y, fog.color.z, fog.color.z);
            context.color_material.enable();
            context.fog.enable();
            context.color(fog.color.x, fog.color.y, fog.color.z, fog.color.z);
        } else {
            context.fog.disable();
            context.color(self.color.x, self.color.y, self.color.z, self.color.w);
        }
        if self.lighting {
            context.lighting.enable();
        } else {
            context.lighting.disable();
        }
        if self.rescale_normal {
            context.rescale_normal.enable();
        } else {
            context.rescale_normal.disable();
        }
        if let Some(material) = self.color_material {
            context.color_material.enable();
            context.color_material(material.face, material.mode)
        } else {
            context.color_material.disable();
        }
        if let Some(stencil) = self.stencil {
            context.stencil.enable();
            if let Some(face) = stencil.face {
                context.stencil_func_separate(face, stencil.func);
                context.stencil_op_separate(face, stencil.stencil_fail, stencil.depth_fail, stencil.depth_pass);
            } else {
                context.stencil_func(stencil.func);
                context.stencil_op(stencil.stencil_fail, stencil.depth_fail, stencil.depth_pass);
            }
        } else {
            context.stencil.disable();
        }
        context.shade_model(self.shade_model);
        context.line_width(self.line_width);
    }
}

impl Clone for GraphicsContext {
    fn clone(&self) -> Self {
        unsafe {
            Self::new()
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BufferBit {
    Stencil = gl::STENCIL_BUFFER_BIT,
    Color = gl::COLOR_BUFFER_BIT,
    Depth = gl::DEPTH_BUFFER_BIT,
}

impl GraphicsContext {
    pub unsafe fn new() -> Self {
        Self {
            alpha: BooleanState,
            auto_normal: BooleanState,
            blend: BooleanState,
            color_material: BooleanState,
            cull_face: BooleanState,
            depth: BooleanState,
            debug_output: BooleanState,
            dither: BooleanState,
            fog: BooleanState,
            lighting: BooleanState,
            rescale_normal: BooleanState,
            stencil: BooleanState,
            scissor: BooleanState,
            texture2d: BooleanState,
            _private: PhantomData,
        }
    }

    #[inline(always)]
    pub fn get_extensions(&self) -> Vec<&CStr> {
        unsafe {
            let mut length = 0;
            gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut length);
            let mut extensions = Vec::with_capacity(length as usize);

            for i in 0..length {
                let ptr = gl::GetStringi(gl::EXTENSIONS, i as _);
                let str = CStr::from_ptr(ptr.cast());
                extensions.push(str);
            }

            extensions
        }
    }

    #[inline(always)]
    pub fn viewport(&self, x: u32, y: u32, w: u32, h: u32) {
        unsafe {
            gl::Viewport(x as _, y as _, w as _, h as _);
        }
    }

    #[inline(always)]
    pub fn color_mask(&self, r: bool, g: bool, b: bool, a: bool) {
        unsafe {
            gl::ColorMask(r as _, g as _, b as _, a as _);
        }
    }

    #[inline(always)]
    pub fn depth_mask(&self, mask: bool) {
        unsafe {
            gl::DepthMask(mask as _);
        }
    }

    #[inline(always)]
    pub fn line_width(&self, width: f32) {
        unsafe {
            gl::LineWidth(width);
        }
    }

    #[inline(always)]
    pub fn color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::Color4f(r, g, b, a);
        }
    }

    #[inline(always)]
    pub fn reset_color(&self) {
        self.color(1.0, 1.0, 1.0, 1.0);
    }

    #[inline(always)]
    pub fn clear(&self, bits: BitFlags<BufferBit>) {
        unsafe {
            gl::Clear(bits.bits());
        }
    }

    #[inline(always)]
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    #[inline(always)]
    pub fn clear_stencil(&self, i: u32) {
        unsafe {
            gl::ClearStencil(i as _)
        }
    }

    #[inline(always)]
    pub fn clear_depth(&self, d: f32) {
        unsafe {
            gl::ClearDepth(d as _);
        }
    }

    #[inline(always)]
    pub fn cull_face(&self, face: Face) {
        unsafe {
            gl::CullFace(face as _);
        }
    }

    #[inline(always)]
    pub fn gen_framebuffer(&self) -> Result<FramebufferId, GlError> {
        let mut id = 0;
        self.gen_framebuffers(std::slice::from_mut(&mut id));
        NonZeroU32::new(id).map(move |i| FramebufferId(i, *self)).ok_or_else(GlError::get)
    }

    #[inline(always)]
    pub fn gen_framebuffers(&self, slots: &mut [u32]) {
        unsafe {
            gl::GenFramebuffers(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn gen_renderbuffer(&self) -> Result<RenderbufferId, GlError> {
        let mut id = 0;
        self.gen_renderbuffers(std::slice::from_mut(&mut id));
        NonZeroU32::new(id).map(move |i| RenderbufferId(i, *self)).ok_or_else(GlError::get)
    }

    #[inline(always)]
    pub fn gen_renderbuffers(&self, slots: &mut [u32]) {
        unsafe {
            gl::GenRenderbuffers(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn gen_buffer<const K: BufferKind>(&self) -> Result<BufferId<K>, GlError> {
        let mut id = 0;
        self.gen_buffers(std::slice::from_mut(&mut id));
        NonZeroU32::new(id).map(move |i| BufferId(i, *self)).ok_or_else(GlError::get)
    }

    #[inline(always)]
    pub fn gen_buffers(&self, slots: &mut [u32]) {
        unsafe {
            gl::GenBuffers(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn gen_vertex_array(&self) -> Result<VertexArrayId, GlError> {
        let mut id = 0;
        self.gen_vertex_arrays(std::slice::from_mut(&mut id));
        NonZeroU32::new(id).map(move |i| VertexArrayId(i, *self)).ok_or_else(GlError::get)
    }

    #[inline(always)]
    pub fn gen_vertex_arrays(&self, slots: &mut [u32]) {
        unsafe {
            gl::GenVertexArrays(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn gen_query(&self) -> Result<QueryId, GlError> {
        let mut id = 0;
        self.gen_queries(std::slice::from_mut(&mut id));
        NonZeroU32::new(id).map(move |i| QueryId(i, *self)).ok_or_else(GlError::get)
    }

    #[inline(always)]
    pub fn gen_queries(&self, slots: &mut[u32]) {
        unsafe {
            gl::GenQueries(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn gen_lists(&self, count: u32) -> Result<DisplayListIter, GlError> {
        unsafe {
            if let Some(start) = NonZeroU32::new(gl::GenLists(count as _)) {
                Ok(DisplayListIter {
                    range: start..start.saturating_add(count),
                    context: *self
                })
            } else {
                Err(GlError::get())
            }
        }
    }

    #[inline(always)]
    pub fn gen_texture(&self) -> Result<TextureId, GlError> {
        let mut id = 0;
        self.gen_textures(std::slice::from_mut(&mut id));
        NonZeroU32::new(id).map(move |i| TextureId(i, *self)).ok_or_else(GlError::get)
    }

    #[inline(always)]
    pub fn gen_textures(&self, slots: &mut [u32]) {
        unsafe {
            gl::GenTextures(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn delete_framebuffers(&self, slots: &mut [u32]) {
        unsafe {
            gl::DeleteFramebuffers(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn delete_renderbuffers(&self, slots: &mut [u32]) {
        unsafe {
            gl::DeleteRenderbuffers(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn delete_buffers(&self, slots: &mut [u32]) {
        unsafe {
            gl::DeleteBuffers(slots.len() as _, slots.as_mut_ptr().cast());
        }
    }

    #[inline(always)]
    pub fn stencil_func(&self, func: StencilFunc) {
        unsafe {
            let reference = func.reference();
            let mask = func.mask();
            let ord = transmute(std::mem::discriminant(&func));
            gl::StencilFunc(ord, reference as _, mask);
        }
    }

    #[inline(always)]
    pub fn stencil_func_separate(&self, face: Face, func: StencilFunc) {
        unsafe {
            let reference = func.reference();
            let mask = func.mask();
            let ord = transmute(std::mem::discriminant(&func));
            gl::StencilFuncSeparate(face as _, ord, reference as _, mask);
        }
    }

    #[inline(always)]
    pub fn stencil_mask(&self, mask: u32) {
        unsafe {
            gl::StencilMask(mask);
        }
    }

    #[inline(always)]
    pub fn stencil_mask_separate(&self, face: Face, mask: u32) {
        unsafe {
            gl::StencilMaskSeparate(face as _, mask);
        }
    }

    #[inline(always)]
    pub fn stencil_op(&self, stencil_fail: StencilOp, depth_fail: StencilOp, depth_pass: StencilOp) {
        unsafe {
            gl::StencilOp(stencil_fail as _, depth_fail as _, depth_pass as _);
        }
    }

    #[inline(always)]
    pub fn stencil_op_separate(&self, face: Face, stencil_fail: StencilOp, depth_fail: StencilOp, depth_pass: StencilOp) {
        unsafe {
            gl::StencilOpSeparate(face as _, stencil_fail as _, depth_fail as _, depth_pass as _);
        }
    }

    #[inline(always)]
    pub fn blend_func(&self, src_color: SrcFactor, dst_color: DstFactor, src_alpha: SrcFactor, dst_alpha: DstFactor) {
        unsafe {
            gl::BlendFuncSeparate(src_color as _, dst_color as _, src_alpha as _, dst_alpha as _);
        }
    }

    #[inline(always)]
    pub fn blend(&self, blend: Blend) {
        self.blend.enable();
        self.blend_func(blend.src_color, blend.dst_color, blend.src_alpha, blend.dst_alpha);
    }

    #[inline(always)]
    pub fn shade_model(&self, mode: ShadeModel) {
        unsafe {
            gl::ShadeModel(mode as _);
        }
    }

    #[inline(always)]
    pub fn alpha_func(&self, func: AlphaFunc) {
        unsafe {
            let reference = func.reference();
            let ord = transmute(std::mem::discriminant(&func));
            gl::AlphaFunc(ord, reference);
        }
    }

    #[inline(always)]
    pub fn depth_func(&self, func: DepthFunc) {
        unsafe {
            gl::DepthFunc(func as _);
        }
    }

    #[inline(always)]
    pub fn light_model(&self, model: LightModel) {
        match model {
            LightModel::Ambient { value } => {
                unsafe {
                    gl::LightModelfv(gl::LIGHT_MODEL_AMBIENT, value.as_ptr());
                }
            }
            LightModel::LocalViewer { value } => {
                unsafe {
                    gl::LightModelf(gl::LIGHT_MODEL_LOCAL_VIEWER, value);
                }
            }
            LightModel::ColorControl { value } => {
                unsafe {
                    gl::LightModeli(gl::LIGHT_MODEL_COLOR_CONTROL, value as _);
                }
            }
            LightModel::TwoSide { value } => {
                unsafe {
                    gl::LightModelf(gl::LIGHT_MODEL_TWO_SIDE, value);
                }
            }
        }
    }

    #[inline(always)]
    pub fn color_material(&self, face: Face, mode: ColorMode) {
        self.color_material.enable();
        unsafe {
            gl::ColorMaterial(face as _, mode as _);
        }
    }

    #[inline(always)]
    pub fn normal(&self, normal: Vector3<f32>) {
        unsafe {
            gl::Normal3fv(normal.as_ptr());
        }
    }

    #[inline(always)]
    pub fn fog_mode(&self, mode: FogMode) {
        unsafe {
            gl::Fogi(gl::FOG_MODE, mode as _);
        }
    }

    #[inline(always)]
    pub fn fog_density(&self, density: f32) {
        unsafe {
            gl::Fogf(gl::FOG_DENSITY, density);
        }
    }

    #[inline(always)]
    pub fn fog_start(&self, start: f32) {
        unsafe {
            gl::Fogf(gl::FOG_START, start);
        }
    }

    #[inline(always)]
    pub fn fog_end(&self, end: f32) {
        unsafe {
            gl::Fogf(gl::FOG_END, end);
        }
    }

    #[inline(always)]
    pub fn fog_color(&self, color: Vector4<f32>) {
        unsafe {
            gl::Fogfv(gl::FOG_COLOR, color.as_ptr());
        }
    }

    #[inline(always)]
    pub fn fog_distance_nv(&self, mode: nv::FogMode) {
        unsafe {
            gl::Fogi(nv::FOG_DISTANCE_MODE_NV, mode as _);
        }
    }

    #[inline(always)]
    pub fn fog_eye_plane_nv(&self, mode: nv::FogEyePlane) {
        unsafe {
            gl::Fogi(nv::FOG_DISTANCE_MODE_NV, mode as _);
        }
    }

    pub fn read_pixels<P>(&self, x: u32, y: u32, width: u32, height: u32, format: PixelFormat, pixels: &mut [P]) where P: Pixel {
        let bytes: &mut [u8] = bytemuck::cast_slice_mut(pixels);
        let size = P::size() * (width * height) as usize;
        assert_eq!(bytes.len(), size);
        unsafe {
            gl::ReadPixels(x as _, y as _, width as _, height as _, format as _, P::gl_type(), bytes.as_mut_ptr().cast())
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct BooleanState<const E: GLenum>;

impl<const E: GLenum> BooleanState<E> {
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        unsafe {
            gl::IsEnabled(E) != 0
        }
    }

    #[inline(always)]
    pub fn enable(&self) {
        unsafe {
            gl::Enable(E);
        }
    }

    #[inline(always)]
    pub fn disable(&self) {
        unsafe {
            gl::Disable(E);
        }
    }
}

pub mod pixel {
    use crate::gl;

    #[repr(u32)]
    pub enum PixelFormat {
        StencilIndex = gl::STENCIL_INDEX,
        DepthComponent = gl::DEPTH_COMPONENT,
        DepthStencil = gl::DEPTH_STENCIL,
        Red = gl::RED,
        Green = gl::GREEN,
        Blue = gl::BLUE,
        RGB = gl::RGB,
        BGR = gl::BGR,
        RGBA = gl::RGBA,
        BGRA = gl::BGRA,
    }
}

pub mod stencil {
    use crate::gl;
    use crate::state::face::Face;

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum StencilFunc {
        Always = gl::ALWAYS,
        Never = gl::NEVER,
        Less { reference: u32, mask: u32 } = gl::LESS,
        LessOrEqual { reference: u32, mask: u32 } = gl::LEQUAL,
        Greater { reference: u32, mask: u32 } = gl::GREATER,
        GreaterOrEqual { reference: u32, mask: u32 } = gl::GEQUAL,
        Equal { reference: u32, mask: u32 } = gl::EQUAL,
        NotEqual { reference: u32, mask: u32 } = gl::NOTEQUAL,
    }

    impl StencilFunc {
        pub fn reference(&self) -> u32 {
            match self {
                StencilFunc::Always => 0,
                StencilFunc::Never => 0,
                StencilFunc::Less { reference, .. } => *reference,
                StencilFunc::LessOrEqual { reference, .. } => *reference,
                StencilFunc::Greater { reference, .. } => *reference,
                StencilFunc::GreaterOrEqual { reference, .. } => *reference,
                StencilFunc::Equal { reference, .. } => *reference,
                StencilFunc::NotEqual { reference, .. } => *reference,
            }
        }

        pub fn mask(&self) -> u32 {
            match self {
                StencilFunc::Always => 0,
                StencilFunc::Never => 0,
                StencilFunc::Less { mask, .. } => *mask,
                StencilFunc::LessOrEqual { mask, .. } => *mask,
                StencilFunc::Greater { mask, .. } => *mask,
                StencilFunc::GreaterOrEqual { mask, .. } => *mask,
                StencilFunc::Equal { mask, .. } => *mask,
                StencilFunc::NotEqual { mask, .. } => *mask,
            }
        }
    }

    #[repr(u32)]
    #[derive(Default, Copy, Clone)]
    pub enum StencilOp {
        #[default]
        Keep = gl::KEEP,
        Zero = gl::ZERO,
        Replace = gl::REPLACE,
        Invert = gl::INVERT,
        Increment = gl::INCR,
        WrappingIncrement = gl::INCR_WRAP,
        Decrement = gl::DECR,
        WrappingDecrement = gl::DECR_WRAP,
    }

    #[derive(Copy, Clone)]
    pub struct Stencil {
        pub face: Option<Face>,
        pub func: StencilFunc,
        pub stencil_fail: StencilOp,
        pub depth_fail: StencilOp,
        pub depth_pass: StencilOp,
    }
}

pub mod alpha {
    use crate::gl;

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum AlphaFunc {
        Always = gl::ALWAYS,
        Never = gl::NEVER,
        Less(f32) = gl::LESS,
        LessOrEqual(f32) = gl::LEQUAL,
        Greater(f32) = gl::GREATER,
        GreaterOrEqual(f32) = gl::GEQUAL,
        Equal(f32) = gl::EQUAL,
        NotEqual(f32) = gl::NOTEQUAL,
    }

    impl AlphaFunc {
        #[inline(always)]
        pub fn reference(&self) -> f32 {
            match self {
                AlphaFunc::Always => 0.0,
                AlphaFunc::Never => 0.0,
                AlphaFunc::Less(r) => *r,
                AlphaFunc::LessOrEqual(r) => *r,
                AlphaFunc::Greater(r) => *r,
                AlphaFunc::GreaterOrEqual(r) => *r,
                AlphaFunc::Equal(r) => *r,
                AlphaFunc::NotEqual(r) => *r,
            }
        }
    }
}

pub mod depth {
    use crate::gl;

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum DepthFunc {
        Always = gl::ALWAYS,
        Never = gl::NEVER,
        Less = gl::LESS,
        LessOrEqual = gl::LEQUAL,
        Greater = gl::GREATER,
        GreaterOrEqual = gl::GEQUAL,
        Equal = gl::EQUAL,
        NotEqual = gl::NOTEQUAL,
    }
}

pub mod fog {
    use cgmath::Vector4;
    use crate::gl;

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum FogMode {
        Linear = gl::LINEAR,
        Exp = gl::EXP,
        Exp2 = gl::EXP2,
    }

    #[derive(Copy, Clone)]
    pub struct Fog {
        pub mode: FogMode,
        pub density: f32,
        pub start: f32,
        pub end: f32,
        pub color: Vector4<f32>,
        pub nv: Option<NvFog>,
    }

    #[derive(Copy, Clone)]
    pub struct NvFog {
        pub distance: super::nv::FogMode,
        pub eye_plane: super::nv::FogEyePlane,
    }
}

pub mod blend {
    use crate::gl;

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum SrcFactor {
        ConstantAlpha = gl::CONSTANT_ALPHA,
        ConstantColor = gl::CONSTANT_COLOR,
        DstAlpha = gl::DST_ALPHA,
        DstColor = gl::DST_COLOR,
        One = gl::ONE,
        OneMinusConstantAlpha = gl::ONE_MINUS_CONSTANT_ALPHA,
        OneMinusConstantColor = gl::ONE_MINUS_CONSTANT_COLOR,
        OneMinusDstAlpha = gl::ONE_MINUS_DST_ALPHA,
        OneMinusDstColor = gl::ONE_MINUS_DST_COLOR,
        OneMinusSrcAlpha = gl::ONE_MINUS_SRC_ALPHA,
        OneMinusSrcColor = gl::ONE_MINUS_SRC_COLOR,
        SrcAlpha = gl::SRC_ALPHA,
        SrcAlphaSaturate = gl::SRC_ALPHA_SATURATE,
        SrcColor = gl::SRC_COLOR,
        Zero = gl::ZERO
    }

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum DstFactor {
        ConstantAlpha = gl::CONSTANT_ALPHA,
        ConstantColor = gl::CONSTANT_COLOR,
        DstAlpha = gl::DST_ALPHA,
        DstColor = gl::DST_COLOR,
        One = gl::ONE,
        OneMinusConstantAlpha = gl::ONE_MINUS_CONSTANT_ALPHA,
        OneMinusConstantColor = gl::ONE_MINUS_CONSTANT_COLOR,
        OneMinusDstAlpha = gl::ONE_MINUS_DST_ALPHA,
        OneMinusDstColor = gl::ONE_MINUS_DST_COLOR,
        OneMinusSrcAlpha = gl::ONE_MINUS_SRC_ALPHA,
        OneMinusSrcColor = gl::ONE_MINUS_SRC_COLOR,
        SrcAlpha = gl::SRC_ALPHA,
        SrcColor = gl::SRC_COLOR,
        Zero = gl::ZERO
    }


    #[derive(Copy, Clone)]
    pub struct Blend {
        pub src_color: SrcFactor,
        pub dst_color: DstFactor,
        pub src_alpha: SrcFactor,
        pub dst_alpha: DstFactor
    }

    impl Default for Blend {
        fn default() -> Self {
            Self {
                src_color: SrcFactor::SrcAlpha,
                dst_color: DstFactor::OneMinusSrcAlpha,
                src_alpha: SrcFactor::One,
                dst_alpha: DstFactor::Zero,
            }
        }
    }
}

pub mod shade {
    use crate::gl;

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum ShadeModel {
        Smooth = gl::SMOOTH,
        Flat = gl::FLAT
    }
}

pub mod draw {
    use crate::gl;

    #[repr(u32)]
    pub enum DrawMode {
        Points = gl::POINTS,
        LineStrip = gl::LINE_STRIP,
        LineLoop = gl::LINE_LOOP,
        Lines = gl::LINES,
        LineStripAdjacency = gl::LINE_STRIP_ADJACENCY,
        TriangleStrip = gl::TRIANGLE_STRIP,
        TriangleFan = gl::TRIANGLE_FAN,
        Triangles = gl::TRIANGLES,
        TriangleStripAdjacency = gl::TRIANGLE_STRIP_ADJACENCY,
        TrianglesAdjacency = gl::TRIANGLES_ADJACENCY,
        Patches = gl::PATCHES,
        Quads = gl::QUADS
    }
}

pub mod light {
    use std::marker::ConstParamTy;
    use crate::gl;

    #[repr(u32)]
    pub enum LightModel {
        Ambient {
            value: [f32; 4]
        } = gl::LIGHT_MODEL_AMBIENT,
        LocalViewer {
            value: f32
        } = gl::LIGHT_MODEL_LOCAL_VIEWER,
        ColorControl {
            value: ColorControl
        } = gl::LIGHT_MODEL_COLOR_CONTROL,
        TwoSide {
            value: f32
        } = gl::LIGHT_MODEL_TWO_SIDE,
    }

    #[repr(u32)]
    pub enum ColorControl {
        SeparateSpecularColor = gl::SEPARATE_SPECULAR_COLOR,
        SingleColor = gl::SINGLE_COLOR
    }

    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq, ConstParamTy)]
    pub enum Light {
        Zero = gl::LIGHT0,
        One = gl::LIGHT1,
        Two = gl::LIGHT2,
        Three = gl::LIGHT3,
        Four = gl::LIGHT4,
        Five = gl::LIGHT5,
        Six = gl::LIGHT6,
        Seven = gl::LIGHT7
    }

    impl Light {
        #[inline(always)]
        pub fn enable(self) {
            unsafe {
                gl::Enable(self as _)
            }
        }

        #[inline(always)]
        pub fn disable(self) {
            unsafe {
                gl::Disable(self as _)
            }
        }

        #[inline(always)]
        pub fn set_position(self, value: &[f32; 4]) {
            unsafe {
                gl::Lightfv(self as _, gl::POSITION, value.as_ptr());
            }
        }

        #[inline(always)]
        pub fn set_ambient(self, value: &[f32; 4]) {
            unsafe {
                gl::Lightfv(self as _, gl::AMBIENT, value.as_ptr());
            }
        }

        #[inline(always)]
        pub fn set_diffuse(self, value: &[f32; 4]) {
            unsafe {
                gl::Lightfv(self as _, gl::DIFFUSE, value.as_ptr());
            }
        }

        #[inline(always)]
        pub fn set_specular(self, value: &[f32; 4]) {
            unsafe {
                gl::Lightfv(self as _, gl::SPECULAR, value.as_ptr());
            }
        }

        #[inline(always)]
        pub fn set_spot_direction(self, value: &[f32; 3]) {
            unsafe {
                gl::Lightfv(self as _, gl::SPOT_DIRECTION, value.as_ptr());
            }
        }

        #[inline(always)]
        pub fn set_spot_exponent(self, value: f32) {
            unsafe {
                gl::Lightf(self as _, gl::SPOT_EXPONENT, value);
            }
        }

        #[inline(always)]
        pub fn set_spot_cutoff(self, value: f32) {
            unsafe {
                gl::Lightf(self as _, gl::SPOT_CUTOFF, value);
            }
        }

        #[inline(always)]
        pub fn set_constant_attenuation(self, value: f32) {
            unsafe {
                gl::Lightf(self as _, gl::CONSTANT_ATTENUATION, value);
            }
        }

        #[inline(always)]
        pub fn set_linear_attenuation(self, value: f32) {
            unsafe {
                gl::Lightf(self as _, gl::LINEAR_ATTENUATION, value);
            }
        }

        #[inline(always)]
        pub fn set_quadratic_attenuation(self, value: f32) {
            unsafe {
                gl::Lightf(self as _, gl::QUADRATIC_ATTENUATION, value);
            }
        }
    }
}

pub mod face {
    use crate::gl;

    #[repr(u32)]
    #[derive(Default, Copy, Clone)]
    pub enum Face {
        Front = gl::FRONT,
        Back = gl::BACK,
        #[default]
        FrontAndBack = gl::FRONT_AND_BACK,
    }
}

pub mod color {
    use crate::gl;
    use crate::state::face::Face;

    #[repr(u32)]
    #[derive(Default, Copy, Clone)]
    pub enum ColorMode {
        Emission = gl::EMISSION,
        Ambient = gl::AMBIENT,
        Diffuse = gl::DIFFUSE,
        Specular = gl::SPECULAR,
        #[default]
        AmbientAndDiffuse = gl::AMBIENT_AND_DIFFUSE
    }

    #[derive(Default, Copy, Clone)]
    pub struct ColorMaterial {
        pub face: Face,
        pub mode: ColorMode,
    }
}

pub mod nv {
    use crate::raw::GLenum;

    pub const FOG_DISTANCE_MODE_NV: GLenum = 0x855A;
    pub const FOG_EYE_RADIAL_NV: GLenum = 0x855B;
    pub const FOG_EYE_PLANE_ABSOLUTE_NV: GLenum = 0x855C;

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum FogMode {
        EyeRadial = FOG_EYE_RADIAL_NV
    }

    #[repr(u32)]
    #[derive(Copy, Clone)]
    pub enum FogEyePlane {
        Absolute = FOG_EYE_PLANE_ABSOLUTE_NV
    }
}