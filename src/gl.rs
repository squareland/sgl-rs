use std::os::raw::c_void;

pub mod types;
pub mod constants;

pub use types::*;
pub use constants::*;

#[inline(never)]
fn metaloadfn(loadfn: &mut dyn FnMut(&'static str) -> *const c_void, symbol: &'static str,
              fallbacks: &[&'static str]) -> *const c_void {
    let mut ptr = loadfn(symbol);
    if ptr.is_null() {
        for &sym in fallbacks {
            ptr = loadfn(sym);
            if !ptr.is_null() { break; }
        }
    }
    ptr
}

macro_rules! gl {
    ($(fn $namespace: ident ($($arg:ident : $ty:ty),*) -> $ret:ty $(, or $fallback: ident)*;)*) => {
        $(
            #[allow(non_snake_case, unused_variables, dead_code)]
            #[inline]
            pub unsafe fn $namespace($($arg: $ty),*) -> $ret { std::mem::transmute::<_, extern "system" fn($($ty),*) -> $ret>($namespace::PTR.f)($($arg),*) }

            #[allow(non_snake_case)]
            pub mod $namespace {
                use std::os::raw::c_void;
                use super::metaloadfn;
                use super::FnPtr;

                pub static mut PTR: FnPtr = FnPtr {
                    f: missing as *const c_void,
                    is_loaded: false
                };

                #[inline(never)]
                fn missing() -> ! {
                    panic!(concat!("gl function `", stringify!($namespace), "` was not loaded"));
                }

                #[inline]
                #[allow(dead_code)]
                pub fn is_loaded() -> bool {
                    unsafe { PTR.is_loaded }
                }

                #[allow(dead_code)]
                pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void {
                    unsafe {
                        let ptr = metaloadfn(&mut loadfn, concat!("gl", stringify!($namespace)), &[$(concat!("gl", stringify!($fallback))),*]);
                        if !ptr.is_null() {
                            PTR = FnPtr {
                                f: ptr,
                                is_loaded: true,
                            };
                        }
                    }
                }
            }
        )*

        #[inline(never)]
        fn inner_load(loadfn: &mut dyn FnMut(&'static str) -> *const c_void) {
            $(
                 $namespace::load_with(&mut *loadfn);
            )*
        }
    };
}

gl! {
    fn Accum(op: GLenum, value: GLfloat) -> ();
    fn ActiveShaderProgram(pipeline: GLuint, program: GLuint) -> ();
    fn ActiveTexture(texture: GLenum) -> (),
      or ActiveTextureARB;
    fn AlphaFunc(func: GLenum, ref_: GLfloat) -> ();
    fn AreTexturesResident(n: GLsizei, textures: *const GLuint, residences: *mut GLboolean) -> GLboolean;
    fn ArrayElement(i: GLint) -> (),
      or ArrayElementEXT;
    fn AttachShader(program: GLuint, shader: GLuint) -> (),
      or AttachObjectARB;
    fn Begin(mode: GLenum) -> ();
    fn BeginConditionalRender(id: GLuint, mode: GLenum) -> (),
      or BeginConditionalRenderNV;
    fn BeginQuery(target: GLenum, id: GLuint) -> (),
      or BeginQueryARB;
    fn BeginQueryIndexed(target: GLenum, index: GLuint, id: GLuint) -> ();
    fn BeginTransformFeedback(primitiveMode: GLenum) -> (),
      or BeginTransformFeedbackEXT,
      or BeginTransformFeedbackNV;
    fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar) -> (),
      or BindAttribLocationARB;
    fn BindBuffer(target: GLenum, buffer: GLuint) -> (),
      or BindBufferARB;
    fn BindBufferBase(target: GLenum, index: GLuint, buffer: GLuint) -> (),
      or BindBufferBaseEXT,
      or BindBufferBaseNV;
    fn BindBufferRange(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> (),
      or BindBufferRangeEXT,
      or BindBufferRangeNV;
    fn BindBuffersBase(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint) -> ();
    fn BindBuffersRange(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr) -> ();
    fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar) -> (),
      or BindFragDataLocationEXT;
    fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar) -> (),
      or BindFragDataLocationIndexedEXT;
    fn BindFramebuffer(target: GLenum, framebuffer: GLuint) -> ();
    fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum) -> ();
    fn BindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> ();
    fn BindProgramPipeline(pipeline: GLuint) -> ();
    fn BindRenderbuffer(target: GLenum, renderbuffer: GLuint) -> ();
    fn BindSampler(unit: GLuint, sampler: GLuint) -> ();
    fn BindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint) -> ();
    fn BindTexture(target: GLenum, texture: GLuint) -> (),
      or BindTextureEXT;
    fn BindTextureUnit(unit: GLuint, texture: GLuint) -> ();
    fn BindTextures(first: GLuint, count: GLsizei, textures: *const GLuint) -> ();
    fn BindTransformFeedback(target: GLenum, id: GLuint) -> ();
    fn BindVertexArray(array: GLuint) -> (),
      or BindVertexArrayOES;
    fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> ();
    fn BindVertexBuffers(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> ();
    fn Bitmap(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte) -> ();
    fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> (),
      or BlendColorEXT;
    fn BlendEquation(mode: GLenum) -> (),
      or BlendEquationEXT;
    fn BlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum) -> (),
      or BlendEquationSeparateEXT;
    fn BlendEquationSeparatei(buf: GLuint, modeRGB: GLenum, modeAlpha: GLenum) -> (),
      or BlendEquationSeparateIndexedAMD,
      or BlendEquationSeparateiARB,
      or BlendEquationSeparateiEXT,
      or BlendEquationSeparateiOES;
    fn BlendEquationi(buf: GLuint, mode: GLenum) -> (),
      or BlendEquationIndexedAMD,
      or BlendEquationiARB,
      or BlendEquationiEXT,
      or BlendEquationiOES;
    fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> ();
    fn BlendFuncSeparate(sfactorRGB: GLenum, dfactorRGB: GLenum, sfactorAlpha: GLenum, dfactorAlpha: GLenum) -> (),
      or BlendFuncSeparateEXT,
      or BlendFuncSeparateINGR;
    fn BlendFuncSeparatei(buf: GLuint, srcRGB: GLenum, dstRGB: GLenum, srcAlpha: GLenum, dstAlpha: GLenum) -> (),
      or BlendFuncSeparateIndexedAMD,
      or BlendFuncSeparateiARB,
      or BlendFuncSeparateiEXT,
      or BlendFuncSeparateiOES;
    fn BlendFunci(buf: GLuint, src: GLenum, dst: GLenum) -> (),
      or BlendFuncIndexedAMD,
      or BlendFunciARB,
      or BlendFunciEXT,
      or BlendFunciOES;
    fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> (),
      or BlitFramebufferEXT,
      or BlitFramebufferNV;
    fn BlitNamedFramebuffer(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum) -> ();
    fn BufferData(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) -> (),
      or BufferDataARB;
    fn BufferStorage(target: GLenum, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) -> (),
      or BufferStorageEXT;
    fn BufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) -> (),
      or BufferSubDataARB;
    fn CallList(list: GLuint) -> ();
    fn CallLists(n: GLsizei, type_: GLenum, lists: *const c_void) -> ();
    fn CheckFramebufferStatus(target: GLenum) -> GLenum,
      or CheckFramebufferStatusEXT;
    fn CheckNamedFramebufferStatus(framebuffer: GLuint, target: GLenum) -> GLenum;
    fn ClampColor(target: GLenum, clamp: GLenum) -> (),
      or ClampColorARB;
    fn Clear(mask: GLbitfield) -> ();
    fn ClearAccum(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> ();
    fn ClearBufferData(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) -> ();
    fn ClearBufferSubData(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) -> ();
    fn ClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> ();
    fn ClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> ();
    fn ClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> ();
    fn ClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> ();
    fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> ();
    fn ClearDepth(depth: GLdouble) -> ();
    fn ClearDepthf(d: GLfloat) -> (),
      or ClearDepthfOES;
    fn ClearIndex(c: GLfloat) -> ();
    fn ClearNamedBufferData(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const c_void) -> ();
    fn ClearNamedBufferSubData(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const c_void) -> ();
    fn ClearNamedFramebufferfi(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) -> ();
    fn ClearNamedFramebufferfv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat) -> ();
    fn ClearNamedFramebufferiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint) -> ();
    fn ClearNamedFramebufferuiv(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint) -> ();
    fn ClearStencil(s: GLint) -> ();
    fn ClearTexImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const c_void) -> (),
      or ClearTexImageEXT;
    fn ClearTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const c_void) -> (),
      or ClearTexSubImageEXT;
    fn ClientActiveTexture(texture: GLenum) -> (),
      or ClientActiveTextureARB;
    fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum,
      or ClientWaitSyncAPPLE;
    fn ClipControl(origin: GLenum, depth: GLenum) -> (),
      or ClipControlEXT;
    fn ClipPlane(plane: GLenum, equation: *const GLdouble) -> ();
    fn Color3b(red: GLbyte, green: GLbyte, blue: GLbyte) -> ();
    fn Color3bv(v: *const GLbyte) -> ();
    fn Color3d(red: GLdouble, green: GLdouble, blue: GLdouble) -> ();
    fn Color3dv(v: *const GLdouble) -> ();
    fn Color3f(red: GLfloat, green: GLfloat, blue: GLfloat) -> ();
    fn Color3fv(v: *const GLfloat) -> ();
    fn Color3i(red: GLint, green: GLint, blue: GLint) -> ();
    fn Color3iv(v: *const GLint) -> ();
    fn Color3s(red: GLshort, green: GLshort, blue: GLshort) -> ();
    fn Color3sv(v: *const GLshort) -> ();
    fn Color3ub(red: GLubyte, green: GLubyte, blue: GLubyte) -> ();
    fn Color3ubv(v: *const GLubyte) -> ();
    fn Color3ui(red: GLuint, green: GLuint, blue: GLuint) -> ();
    fn Color3uiv(v: *const GLuint) -> ();
    fn Color3us(red: GLushort, green: GLushort, blue: GLushort) -> ();
    fn Color3usv(v: *const GLushort) -> ();
    fn Color4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte) -> ();
    fn Color4bv(v: *const GLbyte) -> ();
    fn Color4d(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble) -> ();
    fn Color4dv(v: *const GLdouble) -> ();
    fn Color4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> ();
    fn Color4fv(v: *const GLfloat) -> ();
    fn Color4i(red: GLint, green: GLint, blue: GLint, alpha: GLint) -> ();
    fn Color4iv(v: *const GLint) -> ();
    fn Color4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort) -> ();
    fn Color4sv(v: *const GLshort) -> ();
    fn Color4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte) -> ();
    fn Color4ubv(v: *const GLubyte) -> ();
    fn Color4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint) -> ();
    fn Color4uiv(v: *const GLuint) -> ();
    fn Color4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort) -> ();
    fn Color4usv(v: *const GLushort) -> ();
    fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> ();
    fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean) -> (),
      or ColorMaskIndexedEXT,
      or ColorMaskiEXT,
      or ColorMaskiOES;
    fn ColorMaterial(face: GLenum, mode: GLenum) -> ();
    fn ColorP3ui(type_: GLenum, color: GLuint) -> ();
    fn ColorP3uiv(type_: GLenum, color: *const GLuint) -> ();
    fn ColorP4ui(type_: GLenum, color: GLuint) -> ();
    fn ColorP4uiv(type_: GLenum, color: *const GLuint) -> ();
    fn ColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> ();
    fn CompileShader(shader: GLuint) -> (),
      or CompileShaderARB;
    fn CompressedTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> (),
      or CompressedTexImage1DARB;
    fn CompressedTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> (),
      or CompressedTexImage2DARB;
    fn CompressedTexImage3D(target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const c_void) -> (),
      or CompressedTexImage3DARB;
    fn CompressedTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> (),
      or CompressedTexSubImage1DARB;
    fn CompressedTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> (),
      or CompressedTexSubImage2DARB;
    fn CompressedTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> (),
      or CompressedTexSubImage3DARB;
    fn CompressedTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> ();
    fn CompressedTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> ();
    fn CompressedTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const c_void) -> ();
    fn CopyBufferSubData(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> (),
      or CopyBufferSubDataNV;
    fn CopyImageSubData(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei) -> (),
      or CopyImageSubDataEXT,
      or CopyImageSubDataOES;
    fn CopyNamedBufferSubData(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr) -> ();
    fn CopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: GLenum) -> ();
    fn CopyTexImage1D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, border: GLint) -> (),
      or CopyTexImage1DEXT;
    fn CopyTexImage2D(target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) -> (),
      or CopyTexImage2DEXT;
    fn CopyTexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> (),
      or CopyTexSubImage1DEXT;
    fn CopyTexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),
      or CopyTexSubImage2DEXT;
    fn CopyTexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> (),
      or CopyTexSubImage3DEXT;
    fn CopyTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei) -> ();
    fn CopyTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn CopyTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn CreateBuffers(n: GLsizei, buffers: *mut GLuint) -> ();
    fn CreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> ();
    fn CreateProgram() -> GLuint,
      or CreateProgramObjectARB;
    fn CreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> ();
    fn CreateQueries(target: GLenum, n: GLsizei, ids: *mut GLuint) -> ();
    fn CreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> ();
    fn CreateSamplers(n: GLsizei, samplers: *mut GLuint) -> ();
    fn CreateShader(type_: GLenum) -> GLuint,
      or CreateShaderObjectARB;
    fn CreateShaderProgramv(type_: GLenum, count: GLsizei, strings: *const *const GLchar) -> GLuint;
    fn CreateTextures(target: GLenum, n: GLsizei, textures: *mut GLuint) -> ();
    fn CreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> ();
    fn CreateVertexArrays(n: GLsizei, arrays: *mut GLuint) -> ();
    fn CullFace(mode: GLenum) -> ();
    fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const c_void) -> (),
      or DebugMessageCallbackARB,
      or DebugMessageCallbackKHR;
    fn DebugMessageControl(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean) -> (),
      or DebugMessageControlARB,
      or DebugMessageControlKHR;
    fn DebugMessageInsert(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar) -> (),
      or DebugMessageInsertARB,
      or DebugMessageInsertKHR;
    fn DeleteBuffers(n: GLsizei, buffers: *const GLuint) -> (),
      or DeleteBuffersARB;
    fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) -> (),
      or DeleteFramebuffersEXT;
    fn DeleteLists(list: GLuint, range: GLsizei) -> ();
    fn DeleteProgram(program: GLuint) -> ();
    fn DeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint) -> ();
    fn DeleteQueries(n: GLsizei, ids: *const GLuint) -> (),
      or DeleteQueriesARB;
    fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) -> (),
      or DeleteRenderbuffersEXT;
    fn DeleteSamplers(count: GLsizei, samplers: *const GLuint) -> ();
    fn DeleteShader(shader: GLuint) -> ();
    fn DeleteSync(sync: GLsync) -> (),
      or DeleteSyncAPPLE;
    fn DeleteTextures(n: GLsizei, textures: *const GLuint) -> ();
    fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint) -> (),
      or DeleteTransformFeedbacksNV;
    fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint) -> (),
      or DeleteVertexArraysAPPLE,
      or DeleteVertexArraysOES;
    fn DepthFunc(func: GLenum) -> ();
    fn DepthMask(flag: GLboolean) -> ();
    fn DepthRange(n: GLdouble, f: GLdouble) -> ();
    fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble) -> ();
    fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble) -> ();
    fn DepthRangef(n: GLfloat, f: GLfloat) -> (),
      or DepthRangefOES;
    fn DetachShader(program: GLuint, shader: GLuint) -> (),
      or DetachObjectARB;
    fn Disable(cap: GLenum) -> ();
    fn DisableClientState(array: GLenum) -> ();
    fn DisableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> ();
    fn DisableVertexAttribArray(index: GLuint) -> (),
      or DisableVertexAttribArrayARB;
    fn Disablei(target: GLenum, index: GLuint) -> (),
      or DisableIndexedEXT,
      or DisableiEXT,
      or DisableiNV,
      or DisableiOES;
    fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint) -> ();
    fn DispatchComputeIndirect(indirect: GLintptr) -> ();
    fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) -> (),
      or DrawArraysEXT;
    fn DrawArraysIndirect(mode: GLenum, indirect: *const c_void) -> ();
    fn DrawArraysInstanced(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei) -> (),
      or DrawArraysInstancedANGLE,
      or DrawArraysInstancedARB,
      or DrawArraysInstancedEXT,
      or DrawArraysInstancedNV;
    fn DrawArraysInstancedBaseInstance(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint) -> (),
      or DrawArraysInstancedBaseInstanceEXT;
    fn DrawBuffer(buf: GLenum) -> ();
    fn DrawBuffers(n: GLsizei, bufs: *const GLenum) -> (),
      or DrawBuffersARB,
      or DrawBuffersATI,
      or DrawBuffersEXT;
    fn DrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void) -> ();
    fn DrawElementsBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) -> (),
      or DrawElementsBaseVertexEXT,
      or DrawElementsBaseVertexOES;
    fn DrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void) -> ();
    fn DrawElementsInstanced(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei) -> (),
      or DrawElementsInstancedANGLE,
      or DrawElementsInstancedARB,
      or DrawElementsInstancedEXT,
      or DrawElementsInstancedNV;
    fn DrawElementsInstancedBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, baseinstance: GLuint) -> (),
      or DrawElementsInstancedBaseInstanceEXT;
    fn DrawElementsInstancedBaseVertex(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint) -> (),
      or DrawElementsInstancedBaseVertexEXT,
      or DrawElementsInstancedBaseVertexOES;
    fn DrawElementsInstancedBaseVertexBaseInstance(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const c_void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint) -> (),
      or DrawElementsInstancedBaseVertexBaseInstanceEXT;
    fn DrawPixels(width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> ();
    fn DrawRangeElements(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void) -> (),
      or DrawRangeElementsEXT;
    fn DrawRangeElementsBaseVertex(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const c_void, basevertex: GLint) -> (),
      or DrawRangeElementsBaseVertexEXT,
      or DrawRangeElementsBaseVertexOES;
    fn DrawTransformFeedback(mode: GLenum, id: GLuint) -> (),
      or DrawTransformFeedbackEXT,
      or DrawTransformFeedbackNV;
    fn DrawTransformFeedbackInstanced(mode: GLenum, id: GLuint, instancecount: GLsizei) -> (),
      or DrawTransformFeedbackInstancedEXT;
    fn DrawTransformFeedbackStream(mode: GLenum, id: GLuint, stream: GLuint) -> ();
    fn DrawTransformFeedbackStreamInstanced(mode: GLenum, id: GLuint, stream: GLuint, instancecount: GLsizei) -> ();
    fn EdgeFlag(flag: GLboolean) -> ();
    fn EdgeFlagPointer(stride: GLsizei, pointer: *const c_void) -> ();
    fn EdgeFlagv(flag: *const GLboolean) -> ();
    fn Enable(cap: GLenum) -> ();
    fn EnableClientState(array: GLenum) -> ();
    fn EnableVertexArrayAttrib(vaobj: GLuint, index: GLuint) -> ();
    fn EnableVertexAttribArray(index: GLuint) -> (),
      or EnableVertexAttribArrayARB;
    fn Enablei(target: GLenum, index: GLuint) -> (),
      or EnableIndexedEXT,
      or EnableiEXT,
      or EnableiNV,
      or EnableiOES;
    fn End() -> ();
    fn EndConditionalRender() -> (),
      or EndConditionalRenderNV,
      or EndConditionalRenderNVX;
    fn EndList() -> ();
    fn EndQuery(target: GLenum) -> (),
      or EndQueryARB;
    fn EndQueryIndexed(target: GLenum, index: GLuint) -> ();
    fn EndTransformFeedback() -> (),
      or EndTransformFeedbackEXT,
      or EndTransformFeedbackNV;
    fn EvalCoord1d(u: GLdouble) -> ();
    fn EvalCoord1dv(u: *const GLdouble) -> ();
    fn EvalCoord1f(u: GLfloat) -> ();
    fn EvalCoord1fv(u: *const GLfloat) -> ();
    fn EvalCoord2d(u: GLdouble, v: GLdouble) -> ();
    fn EvalCoord2dv(u: *const GLdouble) -> ();
    fn EvalCoord2f(u: GLfloat, v: GLfloat) -> ();
    fn EvalCoord2fv(u: *const GLfloat) -> ();
    fn EvalMesh1(mode: GLenum, i1: GLint, i2: GLint) -> ();
    fn EvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint) -> ();
    fn EvalPoint1(i: GLint) -> ();
    fn EvalPoint2(i: GLint, j: GLint) -> ();
    fn FeedbackBuffer(size: GLsizei, type_: GLenum, buffer: *mut GLfloat) -> ();
    fn FenceSync(condition: GLenum, flags: GLbitfield) -> GLsync,
      or FenceSyncAPPLE;
    fn Finish() -> ();
    fn Flush() -> ();
    fn FlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr) -> (),
      or FlushMappedBufferRangeAPPLE,
      or FlushMappedBufferRangeEXT;
    fn FlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> ();
    fn FogCoordPointer(type_: GLenum, stride: GLsizei, pointer: *const c_void) -> (),
      or FogCoordPointerEXT;
    fn FogCoordd(coord: GLdouble) -> (),
      or FogCoorddEXT;
    fn FogCoorddv(coord: *const GLdouble) -> (),
      or FogCoorddvEXT;
    fn FogCoordf(coord: GLfloat) -> (),
      or FogCoordfEXT;
    fn FogCoordfv(coord: *const GLfloat) -> (),
      or FogCoordfvEXT;
    fn Fogf(pname: GLenum, param: GLfloat) -> ();
    fn Fogfv(pname: GLenum, params: *const GLfloat) -> ();
    fn Fogi(pname: GLenum, param: GLint) -> ();
    fn Fogiv(pname: GLenum, params: *const GLint) -> ();
    fn FramebufferParameteri(target: GLenum, pname: GLenum, param: GLint) -> ();
    fn FramebufferRenderbuffer(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> (),
      or FramebufferRenderbufferEXT;
    fn FramebufferTexture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) -> (),
      or FramebufferTextureARB,
      or FramebufferTextureEXT,
      or FramebufferTextureOES;
    fn FramebufferTexture1D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> (),
      or FramebufferTexture1DEXT;
    fn FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) -> (),
      or FramebufferTexture2DEXT;
    fn FramebufferTexture3D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint) -> (),
      or FramebufferTexture3DEXT;
    fn FramebufferTextureLayer(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> (),
      or FramebufferTextureLayerARB,
      or FramebufferTextureLayerEXT;
    fn FrontFace(mode: GLenum) -> ();
    fn Frustum(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) -> ();
    fn GenBuffers(n: GLsizei, buffers: *mut GLuint) -> (),
      or GenBuffersARB;
    fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) -> (),
      or GenFramebuffersEXT;
    fn GenLists(range: GLsizei) -> GLuint;
    fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint) -> ();
    fn GenQueries(n: GLsizei, ids: *mut GLuint) -> (),
      or GenQueriesARB;
    fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) -> (),
      or GenRenderbuffersEXT;
    fn GenSamplers(count: GLsizei, samplers: *mut GLuint) -> ();
    fn GenTextures(n: GLsizei, textures: *mut GLuint) -> ();
    fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint) -> (),
      or GenTransformFeedbacksNV;
    fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint) -> (),
      or GenVertexArraysAPPLE,
      or GenVertexArraysOES;
    fn GenerateMipmap(target: GLenum) -> (),
      or GenerateMipmapEXT;
    fn GenerateTextureMipmap(texture: GLuint) -> ();
    fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> (),
      or GetActiveAttribARB;
    fn GetActiveSubroutineName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> ();
    fn GetActiveSubroutineUniformName(program: GLuint, shadertype: GLenum, index: GLuint, bufsize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> ();
    fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: GLenum, index: GLuint, pname: GLenum, values: *mut GLint) -> ();
    fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut GLenum, name: *mut GLchar) -> (),
      or GetActiveUniformARB;
    fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar) -> ();
    fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar) -> ();
    fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint) -> ();
    fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint,
      or GetAttribLocationARB;
    fn GetBooleani_v(target: GLenum, index: GLuint, data: *mut GLboolean) -> (),
      or GetBooleanIndexedvEXT;
    fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> ();
    fn GetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64) -> ();
    fn GetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> (),
      or GetBufferParameterivARB;
    fn GetBufferPointerv(target: GLenum, pname: GLenum, params: *const *mut c_void) -> (),
      or GetBufferPointervARB,
      or GetBufferPointervOES;
    fn GetBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) -> (),
      or GetBufferSubDataARB;
    fn GetClipPlane(plane: GLenum, equation: *mut GLdouble) -> ();
    fn GetCompressedTexImage(target: GLenum, level: GLint, img: *mut c_void) -> (),
      or GetCompressedTexImageARB;
    fn GetCompressedTextureImage(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut c_void) -> ();
    fn GetCompressedTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut c_void) -> ();
    fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint,
      or GetDebugMessageLogARB,
      or GetDebugMessageLogKHR;
    fn GetDoublei_v(target: GLenum, index: GLuint, data: *mut GLdouble) -> (),
      or GetDoubleIndexedvEXT,
      or GetDoublei_vEXT;
    fn GetDoublev(pname: GLenum, data: *mut GLdouble) -> ();
    fn GetError() -> GLenum;
    fn GetFloati_v(target: GLenum, index: GLuint, data: *mut GLfloat) -> (),
      or GetFloatIndexedvEXT,
      or GetFloati_vEXT,
      or GetFloati_vNV,
      or GetFloati_vOES;
    fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> ();
    fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint,
      or GetFragDataIndexEXT;
    fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint,
      or GetFragDataLocationEXT;
    fn GetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) -> (),
      or GetFramebufferAttachmentParameterivEXT;
    fn GetFramebufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetGraphicsResetStatus() -> GLenum,
      or GetGraphicsResetStatusEXT,
      or GetGraphicsResetStatusKHR;
    fn GetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64) -> ();
    fn GetInteger64v(pname: GLenum, data: *mut GLint64) -> (),
      or GetInteger64vAPPLE;
    fn GetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint) -> (),
      or GetIntegerIndexedvEXT;
    fn GetIntegerv(pname: GLenum, data: *mut GLint) -> ();
    fn GetInternalformati64v(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint64) -> ();
    fn GetInternalformativ(target: GLenum, internalformat: GLenum, pname: GLenum, bufSize: GLsizei, params: *mut GLint) -> ();
    fn GetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetLightiv(light: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetMapdv(target: GLenum, query: GLenum, v: *mut GLdouble) -> ();
    fn GetMapfv(target: GLenum, query: GLenum, v: *mut GLfloat) -> ();
    fn GetMapiv(target: GLenum, query: GLenum, v: *mut GLint) -> ();
    fn GetMaterialfv(face: GLenum, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetMaterialiv(face: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetMultisamplefv(pname: GLenum, index: GLuint, val: *mut GLfloat) -> (),
      or GetMultisamplefvNV;
    fn GetNamedBufferParameteri64v(buffer: GLuint, pname: GLenum, params: *mut GLint64) -> ();
    fn GetNamedBufferParameteriv(buffer: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetNamedBufferPointerv(buffer: GLuint, pname: GLenum, params: *const *mut c_void) -> ();
    fn GetNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut c_void) -> ();
    fn GetNamedFramebufferAttachmentParameteriv(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetNamedFramebufferParameteriv(framebuffer: GLuint, pname: GLenum, param: *mut GLint) -> ();
    fn GetNamedRenderbufferParameteriv(renderbuffer: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetObjectLabel(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> (),
      or GetObjectLabelKHR;
    fn GetObjectPtrLabel(ptr: *const c_void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar) -> (),
      or GetObjectPtrLabelKHR;
    fn GetPixelMapfv(map: GLenum, values: *mut GLfloat) -> ();
    fn GetPixelMapuiv(map: GLenum, values: *mut GLuint) -> ();
    fn GetPixelMapusv(map: GLenum, values: *mut GLushort) -> ();
    fn GetPointerv(pname: GLenum, params: *const *mut c_void) -> (),
      or GetPointervEXT,
      or GetPointervKHR;
    fn GetPolygonStipple(mask: *mut GLubyte) -> ();
    fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut c_void) -> (),
      or GetProgramBinaryOES;
    fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> ();
    fn GetProgramInterfaceiv(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> ();
    fn GetProgramPipelineiv(pipeline: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetProgramResourceIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint;
    fn GetProgramResourceLocation(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;
    fn GetProgramResourceLocationIndex(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint;
    fn GetProgramResourceName(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar) -> ();
    fn GetProgramResourceiv(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, bufSize: GLsizei, length: *mut GLsizei, params: *mut GLint) -> ();
    fn GetProgramStageiv(program: GLuint, shadertype: GLenum, pname: GLenum, values: *mut GLint) -> ();
    fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetQueryBufferObjecti64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> ();
    fn GetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> ();
    fn GetQueryBufferObjectui64v(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> ();
    fn GetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr) -> ();
    fn GetQueryIndexediv(target: GLenum, index: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetQueryObjecti64v(id: GLuint, pname: GLenum, params: *mut GLint64) -> (),
      or GetQueryObjecti64vEXT;
    fn GetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint) -> (),
      or GetQueryObjectivARB,
      or GetQueryObjectivEXT;
    fn GetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64) -> (),
      or GetQueryObjectui64vEXT;
    fn GetQueryObjectuiv(id: GLuint, pname: GLenum, params: *mut GLuint) -> (),
      or GetQueryObjectuivARB;
    fn GetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint) -> (),
      or GetQueryivARB;
    fn GetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> (),
      or GetRenderbufferParameterivEXT;
    fn GetSamplerParameterIiv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> (),
      or GetSamplerParameterIivEXT,
      or GetSamplerParameterIivOES;
    fn GetSamplerParameterIuiv(sampler: GLuint, pname: GLenum, params: *mut GLuint) -> (),
      or GetSamplerParameterIuivEXT,
      or GetSamplerParameterIuivOES;
    fn GetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar) -> ();
    fn GetShaderPrecisionFormat(shadertype: GLenum, precisiontype: GLenum, range: *mut GLint, precision: *mut GLint) -> ();
    fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar) -> (),
      or GetShaderSourceARB;
    fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetString(name: GLenum) -> *const GLubyte;
    fn GetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
    fn GetSubroutineIndex(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLuint;
    fn GetSubroutineUniformLocation(program: GLuint, shadertype: GLenum, name: *const GLchar) -> GLint;
    fn GetSynciv(sync: GLsync, pname: GLenum, bufSize: GLsizei, length: *mut GLsizei, values: *mut GLint) -> (),
      or GetSyncivAPPLE;
    fn GetTexEnvfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetTexEnviv(target: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetTexGendv(coord: GLenum, pname: GLenum, params: *mut GLdouble) -> ();
    fn GetTexGenfv(coord: GLenum, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetTexGeniv(coord: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut c_void) -> ();
    fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) -> ();
    fn GetTexParameterIiv(target: GLenum, pname: GLenum, params: *mut GLint) -> (),
      or GetTexParameterIivEXT,
      or GetTexParameterIivOES;
    fn GetTexParameterIuiv(target: GLenum, pname: GLenum, params: *mut GLuint) -> (),
      or GetTexParameterIuivEXT,
      or GetTexParameterIuivOES;
    fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> ();
    fn GetTextureImage(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> ();
    fn GetTextureLevelParameterfv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetTextureLevelParameteriv(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint) -> ();
    fn GetTextureParameterIiv(texture: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetTextureParameterIuiv(texture: GLuint, pname: GLenum, params: *mut GLuint) -> ();
    fn GetTextureParameterfv(texture: GLuint, pname: GLenum, params: *mut GLfloat) -> ();
    fn GetTextureParameteriv(texture: GLuint, pname: GLenum, params: *mut GLint) -> ();
    fn GetTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> ();
    fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar) -> (),
      or GetTransformFeedbackVaryingEXT;
    fn GetTransformFeedbacki64_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64) -> ();
    fn GetTransformFeedbacki_v(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint) -> ();
    fn GetTransformFeedbackiv(xfb: GLuint, pname: GLenum, param: *mut GLint) -> ();
    fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
    fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint) -> ();
    fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint,
      or GetUniformLocationARB;
    fn GetUniformSubroutineuiv(shadertype: GLenum, location: GLint, params: *mut GLuint) -> ();
    fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble) -> ();
    fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat) -> (),
      or GetUniformfvARB;
    fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint) -> (),
      or GetUniformivARB;
    fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint) -> (),
      or GetUniformuivEXT;
    fn GetVertexArrayIndexed64iv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64) -> ();
    fn GetVertexArrayIndexediv(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint) -> ();
    fn GetVertexArrayiv(vaobj: GLuint, pname: GLenum, param: *mut GLint) -> ();
    fn GetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint) -> (),
      or GetVertexAttribIivEXT;
    fn GetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint) -> (),
      or GetVertexAttribIuivEXT;
    fn GetVertexAttribLdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> (),
      or GetVertexAttribLdvEXT;
    fn GetVertexAttribPointerv(index: GLuint, pname: GLenum, pointer: *const *mut c_void) -> (),
      or GetVertexAttribPointervARB,
      or GetVertexAttribPointervNV;
    fn GetVertexAttribdv(index: GLuint, pname: GLenum, params: *mut GLdouble) -> (),
      or GetVertexAttribdvARB,
      or GetVertexAttribdvNV;
    fn GetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat) -> (),
      or GetVertexAttribfvARB,
      or GetVertexAttribfvNV;
    fn GetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint) -> (),
      or GetVertexAttribivARB,
      or GetVertexAttribivNV;
    fn GetnColorTable(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, table: *mut c_void) -> ();
    fn GetnCompressedTexImage(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut c_void) -> ();
    fn GetnConvolutionFilter(target: GLenum, format: GLenum, type_: GLenum, bufSize: GLsizei, image: *mut c_void) -> ();
    fn GetnHistogram(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) -> ();
    fn GetnMapdv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble) -> ();
    fn GetnMapfv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat) -> ();
    fn GetnMapiv(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint) -> ();
    fn GetnMinmax(target: GLenum, reset: GLboolean, format: GLenum, type_: GLenum, bufSize: GLsizei, values: *mut c_void) -> ();
    fn GetnPixelMapfv(map: GLenum, bufSize: GLsizei, values: *mut GLfloat) -> ();
    fn GetnPixelMapuiv(map: GLenum, bufSize: GLsizei, values: *mut GLuint) -> ();
    fn GetnPixelMapusv(map: GLenum, bufSize: GLsizei, values: *mut GLushort) -> ();
    fn GetnPolygonStipple(bufSize: GLsizei, pattern: *mut GLubyte) -> ();
    fn GetnSeparableFilter(target: GLenum, format: GLenum, type_: GLenum, rowBufSize: GLsizei, row: *mut c_void, columnBufSize: GLsizei, column: *mut c_void, span: *mut c_void) -> ();
    fn GetnTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut c_void) -> ();
    fn GetnUniformdv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble) -> ();
    fn GetnUniformfv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat) -> (),
      or GetnUniformfvEXT,
      or GetnUniformfvKHR;
    fn GetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint) -> (),
      or GetnUniformivEXT,
      or GetnUniformivKHR;
    fn GetnUniformuiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint) -> (),
      or GetnUniformuivKHR;
    fn Hint(target: GLenum, mode: GLenum) -> ();
    fn IndexMask(mask: GLuint) -> ();
    fn IndexPointer(type_: GLenum, stride: GLsizei, pointer: *const c_void) -> ();
    fn Indexd(c: GLdouble) -> ();
    fn Indexdv(c: *const GLdouble) -> ();
    fn Indexf(c: GLfloat) -> ();
    fn Indexfv(c: *const GLfloat) -> ();
    fn Indexi(c: GLint) -> ();
    fn Indexiv(c: *const GLint) -> ();
    fn Indexs(c: GLshort) -> ();
    fn Indexsv(c: *const GLshort) -> ();
    fn Indexub(c: GLubyte) -> ();
    fn Indexubv(c: *const GLubyte) -> ();
    fn InitNames() -> ();
    fn InterleavedArrays(format: GLenum, stride: GLsizei, pointer: *const c_void) -> ();
    fn InvalidateBufferData(buffer: GLuint) -> ();
    fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr) -> ();
    fn InvalidateFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum) -> ();
    fn InvalidateNamedFramebufferData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum) -> ();
    fn InvalidateNamedFramebufferSubData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn InvalidateSubFramebuffer(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn InvalidateTexImage(texture: GLuint, level: GLint) -> ();
    fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei) -> ();
    fn IsBuffer(buffer: GLuint) -> GLboolean,
      or IsBufferARB;
    fn IsEnabled(cap: GLenum) -> GLboolean;
    fn IsEnabledi(target: GLenum, index: GLuint) -> GLboolean,
      or IsEnabledIndexedEXT,
      or IsEnablediEXT,
      or IsEnablediNV,
      or IsEnablediOES;
    fn IsFramebuffer(framebuffer: GLuint) -> GLboolean,
      or IsFramebufferEXT;
    fn IsList(list: GLuint) -> GLboolean;
    fn IsProgram(program: GLuint) -> GLboolean;
    fn IsProgramPipeline(pipeline: GLuint) -> GLboolean;
    fn IsQuery(id: GLuint) -> GLboolean,
      or IsQueryARB;
    fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean,
      or IsRenderbufferEXT;
    fn IsSampler(sampler: GLuint) -> GLboolean;
    fn IsShader(shader: GLuint) -> GLboolean;
    fn IsSync(sync: GLsync) -> GLboolean,
      or IsSyncAPPLE;
    fn IsTexture(texture: GLuint) -> GLboolean;
    fn IsTransformFeedback(id: GLuint) -> GLboolean,
      or IsTransformFeedbackNV;
    fn IsVertexArray(array: GLuint) -> GLboolean,
      or IsVertexArrayAPPLE,
      or IsVertexArrayOES;
    fn LightModelf(pname: GLenum, param: GLfloat) -> ();
    fn LightModelfv(pname: GLenum, params: *const GLfloat) -> ();
    fn LightModeli(pname: GLenum, param: GLint) -> ();
    fn LightModeliv(pname: GLenum, params: *const GLint) -> ();
    fn Lightf(light: GLenum, pname: GLenum, param: GLfloat) -> ();
    fn Lightfv(light: GLenum, pname: GLenum, params: *const GLfloat) -> ();
    fn Lighti(light: GLenum, pname: GLenum, param: GLint) -> ();
    fn Lightiv(light: GLenum, pname: GLenum, params: *const GLint) -> ();
    fn LineStipple(factor: GLint, pattern: GLushort) -> ();
    fn LineWidth(width: GLfloat) -> ();
    fn LinkProgram(program: GLuint) -> (),
      or LinkProgramARB;
    fn ListBase(base: GLuint) -> ();
    fn LoadIdentity() -> ();
    fn LoadMatrixd(m: *const GLdouble) -> ();
    fn LoadMatrixf(m: *const GLfloat) -> ();
    fn LoadName(name: GLuint) -> ();
    fn LoadTransposeMatrixd(m: *const GLdouble) -> (),
      or LoadTransposeMatrixdARB;
    fn LoadTransposeMatrixf(m: *const GLfloat) -> (),
      or LoadTransposeMatrixfARB;
    fn LogicOp(opcode: GLenum) -> ();
    fn Map1d(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble) -> ();
    fn Map1f(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat) -> ();
    fn Map2d(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble) -> ();
    fn Map2f(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat) -> ();
    fn MapBuffer(target: GLenum, access: GLenum) -> *mut c_void,
      or MapBufferARB,
      or MapBufferOES;
    fn MapBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void,
      or MapBufferRangeEXT;
    fn MapGrid1d(un: GLint, u1: GLdouble, u2: GLdouble) -> ();
    fn MapGrid1f(un: GLint, u1: GLfloat, u2: GLfloat) -> ();
    fn MapGrid2d(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble) -> ();
    fn MapGrid2f(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat) -> ();
    fn MapNamedBuffer(buffer: GLuint, access: GLenum) -> *mut c_void;
    fn MapNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut c_void;
    fn Materialf(face: GLenum, pname: GLenum, param: GLfloat) -> ();
    fn Materialfv(face: GLenum, pname: GLenum, params: *const GLfloat) -> ();
    fn Materiali(face: GLenum, pname: GLenum, param: GLint) -> ();
    fn Materialiv(face: GLenum, pname: GLenum, params: *const GLint) -> ();
    fn MatrixMode(mode: GLenum) -> ();
    fn MemoryBarrier(barriers: GLbitfield) -> (),
      or MemoryBarrierEXT;
    fn MemoryBarrierByRegion(barriers: GLbitfield) -> ();
    fn MinSampleShading(value: GLfloat) -> (),
      or MinSampleShadingARB,
      or MinSampleShadingOES;
    fn MultMatrixd(m: *const GLdouble) -> ();
    fn MultMatrixf(m: *const GLfloat) -> ();
    fn MultTransposeMatrixd(m: *const GLdouble) -> (),
      or MultTransposeMatrixdARB;
    fn MultTransposeMatrixf(m: *const GLfloat) -> (),
      or MultTransposeMatrixfARB;
    fn MultiDrawArrays(mode: GLenum, first: *const GLint, count: *const GLsizei, drawcount: GLsizei) -> (),
      or MultiDrawArraysEXT;
    fn MultiDrawArraysIndirect(mode: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) -> (),
      or MultiDrawArraysIndirectAMD,
      or MultiDrawArraysIndirectEXT;
    fn MultiDrawElements(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei) -> (),
      or MultiDrawElementsEXT;
    fn MultiDrawElementsBaseVertex(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const c_void, drawcount: GLsizei, basevertex: *const GLint) -> (),
      or MultiDrawElementsBaseVertexEXT;
    fn MultiDrawElementsIndirect(mode: GLenum, type_: GLenum, indirect: *const c_void, drawcount: GLsizei, stride: GLsizei) -> (),
      or MultiDrawElementsIndirectAMD,
      or MultiDrawElementsIndirectEXT;
    fn MultiTexCoord1d(target: GLenum, s: GLdouble) -> (),
      or MultiTexCoord1dARB;
    fn MultiTexCoord1dv(target: GLenum, v: *const GLdouble) -> (),
      or MultiTexCoord1dvARB;
    fn MultiTexCoord1f(target: GLenum, s: GLfloat) -> (),
      or MultiTexCoord1fARB;
    fn MultiTexCoord1fv(target: GLenum, v: *const GLfloat) -> (),
      or MultiTexCoord1fvARB;
    fn MultiTexCoord1i(target: GLenum, s: GLint) -> (),
      or MultiTexCoord1iARB;
    fn MultiTexCoord1iv(target: GLenum, v: *const GLint) -> (),
      or MultiTexCoord1ivARB;
    fn MultiTexCoord1s(target: GLenum, s: GLshort) -> (),
      or MultiTexCoord1sARB;
    fn MultiTexCoord1sv(target: GLenum, v: *const GLshort) -> (),
      or MultiTexCoord1svARB;
    fn MultiTexCoord2d(target: GLenum, s: GLdouble, t: GLdouble) -> (),
      or MultiTexCoord2dARB;
    fn MultiTexCoord2dv(target: GLenum, v: *const GLdouble) -> (),
      or MultiTexCoord2dvARB;
    fn MultiTexCoord2f(target: GLenum, s: GLfloat, t: GLfloat) -> (),
      or MultiTexCoord2fARB;
    fn MultiTexCoord2fv(target: GLenum, v: *const GLfloat) -> (),
      or MultiTexCoord2fvARB;
    fn MultiTexCoord2i(target: GLenum, s: GLint, t: GLint) -> (),
      or MultiTexCoord2iARB;
    fn MultiTexCoord2iv(target: GLenum, v: *const GLint) -> (),
      or MultiTexCoord2ivARB;
    fn MultiTexCoord2s(target: GLenum, s: GLshort, t: GLshort) -> (),
      or MultiTexCoord2sARB;
    fn MultiTexCoord2sv(target: GLenum, v: *const GLshort) -> (),
      or MultiTexCoord2svARB;
    fn MultiTexCoord3d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble) -> (),
      or MultiTexCoord3dARB;
    fn MultiTexCoord3dv(target: GLenum, v: *const GLdouble) -> (),
      or MultiTexCoord3dvARB;
    fn MultiTexCoord3f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat) -> (),
      or MultiTexCoord3fARB;
    fn MultiTexCoord3fv(target: GLenum, v: *const GLfloat) -> (),
      or MultiTexCoord3fvARB;
    fn MultiTexCoord3i(target: GLenum, s: GLint, t: GLint, r: GLint) -> (),
      or MultiTexCoord3iARB;
    fn MultiTexCoord3iv(target: GLenum, v: *const GLint) -> (),
      or MultiTexCoord3ivARB;
    fn MultiTexCoord3s(target: GLenum, s: GLshort, t: GLshort, r: GLshort) -> (),
      or MultiTexCoord3sARB;
    fn MultiTexCoord3sv(target: GLenum, v: *const GLshort) -> (),
      or MultiTexCoord3svARB;
    fn MultiTexCoord4d(target: GLenum, s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) -> (),
      or MultiTexCoord4dARB;
    fn MultiTexCoord4dv(target: GLenum, v: *const GLdouble) -> (),
      or MultiTexCoord4dvARB;
    fn MultiTexCoord4f(target: GLenum, s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) -> (),
      or MultiTexCoord4fARB;
    fn MultiTexCoord4fv(target: GLenum, v: *const GLfloat) -> (),
      or MultiTexCoord4fvARB;
    fn MultiTexCoord4i(target: GLenum, s: GLint, t: GLint, r: GLint, q: GLint) -> (),
      or MultiTexCoord4iARB;
    fn MultiTexCoord4iv(target: GLenum, v: *const GLint) -> (),
      or MultiTexCoord4ivARB;
    fn MultiTexCoord4s(target: GLenum, s: GLshort, t: GLshort, r: GLshort, q: GLshort) -> (),
      or MultiTexCoord4sARB;
    fn MultiTexCoord4sv(target: GLenum, v: *const GLshort) -> (),
      or MultiTexCoord4svARB;
    fn MultiTexCoordP1ui(texture: GLenum, type_: GLenum, coords: GLuint) -> ();
    fn MultiTexCoordP1uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> ();
    fn MultiTexCoordP2ui(texture: GLenum, type_: GLenum, coords: GLuint) -> ();
    fn MultiTexCoordP2uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> ();
    fn MultiTexCoordP3ui(texture: GLenum, type_: GLenum, coords: GLuint) -> ();
    fn MultiTexCoordP3uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> ();
    fn MultiTexCoordP4ui(texture: GLenum, type_: GLenum, coords: GLuint) -> ();
    fn MultiTexCoordP4uiv(texture: GLenum, type_: GLenum, coords: *const GLuint) -> ();
    fn NamedBufferData(buffer: GLuint, size: GLsizeiptr, data: *const c_void, usage: GLenum) -> ();
    fn NamedBufferStorage(buffer: GLuint, size: GLsizeiptr, data: *const c_void, flags: GLbitfield) -> (),
      or NamedBufferStorageEXT;
    fn NamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const c_void) -> (),
      or NamedBufferSubDataEXT;
    fn NamedFramebufferDrawBuffer(framebuffer: GLuint, buf: GLenum) -> ();
    fn NamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum) -> ();
    fn NamedFramebufferParameteri(framebuffer: GLuint, pname: GLenum, param: GLint) -> ();
    fn NamedFramebufferReadBuffer(framebuffer: GLuint, src: GLenum) -> ();
    fn NamedFramebufferRenderbuffer(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint) -> ();
    fn NamedFramebufferTexture(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint) -> ();
    fn NamedFramebufferTextureLayer(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint) -> ();
    fn NamedRenderbufferStorage(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei) -> ();
    fn NamedRenderbufferStorageMultisample(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> ();
    fn NewList(list: GLuint, mode: GLenum) -> ();
    fn Normal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte) -> ();
    fn Normal3bv(v: *const GLbyte) -> ();
    fn Normal3d(nx: GLdouble, ny: GLdouble, nz: GLdouble) -> ();
    fn Normal3dv(v: *const GLdouble) -> ();
    fn Normal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat) -> ();
    fn Normal3fv(v: *const GLfloat) -> ();
    fn Normal3i(nx: GLint, ny: GLint, nz: GLint) -> ();
    fn Normal3iv(v: *const GLint) -> ();
    fn Normal3s(nx: GLshort, ny: GLshort, nz: GLshort) -> ();
    fn Normal3sv(v: *const GLshort) -> ();
    fn NormalP3ui(type_: GLenum, coords: GLuint) -> ();
    fn NormalP3uiv(type_: GLenum, coords: *const GLuint) -> ();
    fn NormalPointer(type_: GLenum, stride: GLsizei, pointer: *const c_void) -> ();
    fn ObjectLabel(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar) -> (),
      or ObjectLabelKHR;
    fn ObjectPtrLabel(ptr: *const c_void, length: GLsizei, label: *const GLchar) -> (),
      or ObjectPtrLabelKHR;
    fn Ortho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) -> ();
    fn PassThrough(token: GLfloat) -> ();
    fn PatchParameterfv(pname: GLenum, values: *const GLfloat) -> ();
    fn PatchParameteri(pname: GLenum, value: GLint) -> (),
      or PatchParameteriEXT,
      or PatchParameteriOES;
    fn PauseTransformFeedback() -> (),
      or PauseTransformFeedbackNV;
    fn PixelMapfv(map: GLenum, mapsize: GLsizei, values: *const GLfloat) -> ();
    fn PixelMapuiv(map: GLenum, mapsize: GLsizei, values: *const GLuint) -> ();
    fn PixelMapusv(map: GLenum, mapsize: GLsizei, values: *const GLushort) -> ();
    fn PixelStoref(pname: GLenum, param: GLfloat) -> ();
    fn PixelStorei(pname: GLenum, param: GLint) -> ();
    fn PixelTransferf(pname: GLenum, param: GLfloat) -> ();
    fn PixelTransferi(pname: GLenum, param: GLint) -> ();
    fn PixelZoom(xfactor: GLfloat, yfactor: GLfloat) -> ();
    fn PointParameterf(pname: GLenum, param: GLfloat) -> (),
      or PointParameterfARB,
      or PointParameterfEXT,
      or PointParameterfSGIS;
    fn PointParameterfv(pname: GLenum, params: *const GLfloat) -> (),
      or PointParameterfvARB,
      or PointParameterfvEXT,
      or PointParameterfvSGIS;
    fn PointParameteri(pname: GLenum, param: GLint) -> (),
      or PointParameteriNV;
    fn PointParameteriv(pname: GLenum, params: *const GLint) -> (),
      or PointParameterivNV;
    fn PointSize(size: GLfloat) -> ();
    fn PolygonMode(face: GLenum, mode: GLenum) -> (),
      or PolygonModeNV;
    fn PolygonOffset(factor: GLfloat, units: GLfloat) -> ();
    fn PolygonStipple(mask: *const GLubyte) -> ();
    fn PopAttrib() -> ();
    fn PopClientAttrib() -> ();
    fn PopDebugGroup() -> (),
      or PopDebugGroupKHR;
    fn PopMatrix() -> ();
    fn PopName() -> ();
    fn PrimitiveRestartIndex(index: GLuint) -> ();
    fn PrioritizeTextures(n: GLsizei, textures: *const GLuint, priorities: *const GLfloat) -> (),
      or PrioritizeTexturesEXT;
    fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const c_void, length: GLsizei) -> (),
      or ProgramBinaryOES;
    fn ProgramParameteri(program: GLuint, pname: GLenum, value: GLint) -> (),
      or ProgramParameteriARB,
      or ProgramParameteriEXT;
    fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble) -> ();
    fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat) -> (),
      or ProgramUniform1fEXT;
    fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or ProgramUniform1fvEXT;
    fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint) -> (),
      or ProgramUniform1iEXT;
    fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> (),
      or ProgramUniform1ivEXT;
    fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint) -> (),
      or ProgramUniform1uiEXT;
    fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or ProgramUniform1uivEXT;
    fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble) -> ();
    fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat) -> (),
      or ProgramUniform2fEXT;
    fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or ProgramUniform2fvEXT;
    fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint) -> (),
      or ProgramUniform2iEXT;
    fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> (),
      or ProgramUniform2ivEXT;
    fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint) -> (),
      or ProgramUniform2uiEXT;
    fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or ProgramUniform2uivEXT;
    fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble) -> ();
    fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> (),
      or ProgramUniform3fEXT;
    fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or ProgramUniform3fvEXT;
    fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint) -> (),
      or ProgramUniform3iEXT;
    fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> (),
      or ProgramUniform3ivEXT;
    fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> (),
      or ProgramUniform3uiEXT;
    fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or ProgramUniform3uivEXT;
    fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble) -> ();
    fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> (),
      or ProgramUniform4fEXT;
    fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or ProgramUniform4fvEXT;
    fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> (),
      or ProgramUniform4iEXT;
    fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint) -> (),
      or ProgramUniform4ivEXT;
    fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> (),
      or ProgramUniform4uiEXT;
    fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or ProgramUniform4uivEXT;
    fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix2fvEXT;
    fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix2x3fvEXT;
    fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix2x4fvEXT;
    fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix3fvEXT;
    fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix3x2fvEXT;
    fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix3x4fvEXT;
    fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix4fvEXT;
    fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix4x2fvEXT;
    fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or ProgramUniformMatrix4x3fvEXT;
    fn ProvokingVertex(mode: GLenum) -> (),
      or ProvokingVertexEXT;
    fn PushAttrib(mask: GLbitfield) -> ();
    fn PushClientAttrib(mask: GLbitfield) -> ();
    fn PushDebugGroup(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar) -> (),
      or PushDebugGroupKHR;
    fn PushMatrix() -> ();
    fn PushName(name: GLuint) -> ();
    fn QueryCounter(id: GLuint, target: GLenum) -> (),
      or QueryCounterEXT;
    fn RasterPos2d(x: GLdouble, y: GLdouble) -> ();
    fn RasterPos2dv(v: *const GLdouble) -> ();
    fn RasterPos2f(x: GLfloat, y: GLfloat) -> ();
    fn RasterPos2fv(v: *const GLfloat) -> ();
    fn RasterPos2i(x: GLint, y: GLint) -> ();
    fn RasterPos2iv(v: *const GLint) -> ();
    fn RasterPos2s(x: GLshort, y: GLshort) -> ();
    fn RasterPos2sv(v: *const GLshort) -> ();
    fn RasterPos3d(x: GLdouble, y: GLdouble, z: GLdouble) -> ();
    fn RasterPos3dv(v: *const GLdouble) -> ();
    fn RasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat) -> ();
    fn RasterPos3fv(v: *const GLfloat) -> ();
    fn RasterPos3i(x: GLint, y: GLint, z: GLint) -> ();
    fn RasterPos3iv(v: *const GLint) -> ();
    fn RasterPos3s(x: GLshort, y: GLshort, z: GLshort) -> ();
    fn RasterPos3sv(v: *const GLshort) -> ();
    fn RasterPos4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> ();
    fn RasterPos4dv(v: *const GLdouble) -> ();
    fn RasterPos4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> ();
    fn RasterPos4fv(v: *const GLfloat) -> ();
    fn RasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint) -> ();
    fn RasterPos4iv(v: *const GLint) -> ();
    fn RasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> ();
    fn RasterPos4sv(v: *const GLshort) -> ();
    fn ReadBuffer(src: GLenum) -> ();
    fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut c_void) -> ();
    fn ReadnPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut c_void) -> (),
      or ReadnPixelsARB,
      or ReadnPixelsEXT,
      or ReadnPixelsKHR;
    fn Rectd(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble) -> ();
    fn Rectdv(v1: *const GLdouble, v2: *const GLdouble) -> ();
    fn Rectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat) -> ();
    fn Rectfv(v1: *const GLfloat, v2: *const GLfloat) -> ();
    fn Recti(x1: GLint, y1: GLint, x2: GLint, y2: GLint) -> ();
    fn Rectiv(v1: *const GLint, v2: *const GLint) -> ();
    fn Rects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort) -> ();
    fn Rectsv(v1: *const GLshort, v2: *const GLshort) -> ();
    fn ReleaseShaderCompiler() -> ();
    fn RenderMode(mode: GLenum) -> GLint;
    fn RenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) -> (),
      or RenderbufferStorageEXT;
    fn RenderbufferStorageMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> (),
      or RenderbufferStorageMultisampleEXT,
      or RenderbufferStorageMultisampleNV;
    fn ResumeTransformFeedback() -> (),
      or ResumeTransformFeedbackNV;
    fn Rotated(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble) -> ();
    fn Rotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) -> ();
    fn SampleCoverage(value: GLfloat, invert: GLboolean) -> (),
      or SampleCoverageARB;
    fn SampleMaski(maskNumber: GLuint, mask: GLbitfield) -> ();
    fn SamplerParameterIiv(sampler: GLuint, pname: GLenum, param: *const GLint) -> (),
      or SamplerParameterIivEXT,
      or SamplerParameterIivOES;
    fn SamplerParameterIuiv(sampler: GLuint, pname: GLenum, param: *const GLuint) -> (),
      or SamplerParameterIuivEXT,
      or SamplerParameterIuivOES;
    fn SamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat) -> ();
    fn SamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat) -> ();
    fn SamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint) -> ();
    fn SamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint) -> ();
    fn Scaled(x: GLdouble, y: GLdouble, z: GLdouble) -> ();
    fn Scalef(x: GLfloat, y: GLfloat, z: GLfloat) -> ();
    fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn ScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint) -> (),
      or ScissorArrayvNV,
      or ScissorArrayvOES;
    fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei) -> (),
      or ScissorIndexedNV,
      or ScissorIndexedOES;
    fn ScissorIndexedv(index: GLuint, v: *const GLint) -> (),
      or ScissorIndexedvNV,
      or ScissorIndexedvOES;
    fn SecondaryColor3b(red: GLbyte, green: GLbyte, blue: GLbyte) -> (),
      or SecondaryColor3bEXT;
    fn SecondaryColor3bv(v: *const GLbyte) -> (),
      or SecondaryColor3bvEXT;
    fn SecondaryColor3d(red: GLdouble, green: GLdouble, blue: GLdouble) -> (),
      or SecondaryColor3dEXT;
    fn SecondaryColor3dv(v: *const GLdouble) -> (),
      or SecondaryColor3dvEXT;
    fn SecondaryColor3f(red: GLfloat, green: GLfloat, blue: GLfloat) -> (),
      or SecondaryColor3fEXT;
    fn SecondaryColor3fv(v: *const GLfloat) -> (),
      or SecondaryColor3fvEXT;
    fn SecondaryColor3i(red: GLint, green: GLint, blue: GLint) -> (),
      or SecondaryColor3iEXT;
    fn SecondaryColor3iv(v: *const GLint) -> (),
      or SecondaryColor3ivEXT;
    fn SecondaryColor3s(red: GLshort, green: GLshort, blue: GLshort) -> (),
      or SecondaryColor3sEXT;
    fn SecondaryColor3sv(v: *const GLshort) -> (),
      or SecondaryColor3svEXT;
    fn SecondaryColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte) -> (),
      or SecondaryColor3ubEXT;
    fn SecondaryColor3ubv(v: *const GLubyte) -> (),
      or SecondaryColor3ubvEXT;
    fn SecondaryColor3ui(red: GLuint, green: GLuint, blue: GLuint) -> (),
      or SecondaryColor3uiEXT;
    fn SecondaryColor3uiv(v: *const GLuint) -> (),
      or SecondaryColor3uivEXT;
    fn SecondaryColor3us(red: GLushort, green: GLushort, blue: GLushort) -> (),
      or SecondaryColor3usEXT;
    fn SecondaryColor3usv(v: *const GLushort) -> (),
      or SecondaryColor3usvEXT;
    fn SecondaryColorP3ui(type_: GLenum, color: GLuint) -> ();
    fn SecondaryColorP3uiv(type_: GLenum, color: *const GLuint) -> ();
    fn SecondaryColorPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> (),
      or SecondaryColorPointerEXT;
    fn SelectBuffer(size: GLsizei, buffer: *mut GLuint) -> ();
    fn ShadeModel(mode: GLenum) -> ();
    fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryformat: GLenum, binary: *const c_void, length: GLsizei) -> ();
    fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) -> (),
      or ShaderSourceARB;
    fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint) -> ();
    fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> ();
    fn StencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) -> ();
    fn StencilMask(mask: GLuint) -> ();
    fn StencilMaskSeparate(face: GLenum, mask: GLuint) -> ();
    fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> ();
    fn StencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum) -> (),
      or StencilOpSeparateATI;
    fn TexBuffer(target: GLenum, internalformat: GLenum, buffer: GLuint) -> (),
      or TexBufferARB,
      or TexBufferEXT,
      or TexBufferOES;
    fn TexBufferRange(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> (),
      or TexBufferRangeEXT,
      or TexBufferRangeOES;
    fn TexCoord1d(s: GLdouble) -> ();
    fn TexCoord1dv(v: *const GLdouble) -> ();
    fn TexCoord1f(s: GLfloat) -> ();
    fn TexCoord1fv(v: *const GLfloat) -> ();
    fn TexCoord1i(s: GLint) -> ();
    fn TexCoord1iv(v: *const GLint) -> ();
    fn TexCoord1s(s: GLshort) -> ();
    fn TexCoord1sv(v: *const GLshort) -> ();
    fn TexCoord2d(s: GLdouble, t: GLdouble) -> ();
    fn TexCoord2dv(v: *const GLdouble) -> ();
    fn TexCoord2f(s: GLfloat, t: GLfloat) -> ();
    fn TexCoord2fv(v: *const GLfloat) -> ();
    fn TexCoord2i(s: GLint, t: GLint) -> ();
    fn TexCoord2iv(v: *const GLint) -> ();
    fn TexCoord2s(s: GLshort, t: GLshort) -> ();
    fn TexCoord2sv(v: *const GLshort) -> ();
    fn TexCoord3d(s: GLdouble, t: GLdouble, r: GLdouble) -> ();
    fn TexCoord3dv(v: *const GLdouble) -> ();
    fn TexCoord3f(s: GLfloat, t: GLfloat, r: GLfloat) -> ();
    fn TexCoord3fv(v: *const GLfloat) -> ();
    fn TexCoord3i(s: GLint, t: GLint, r: GLint) -> ();
    fn TexCoord3iv(v: *const GLint) -> ();
    fn TexCoord3s(s: GLshort, t: GLshort, r: GLshort) -> ();
    fn TexCoord3sv(v: *const GLshort) -> ();
    fn TexCoord4d(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) -> ();
    fn TexCoord4dv(v: *const GLdouble) -> ();
    fn TexCoord4f(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) -> ();
    fn TexCoord4fv(v: *const GLfloat) -> ();
    fn TexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint) -> ();
    fn TexCoord4iv(v: *const GLint) -> ();
    fn TexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort) -> ();
    fn TexCoord4sv(v: *const GLshort) -> ();
    fn TexCoordP1ui(type_: GLenum, coords: GLuint) -> ();
    fn TexCoordP1uiv(type_: GLenum, coords: *const GLuint) -> ();
    fn TexCoordP2ui(type_: GLenum, coords: GLuint) -> ();
    fn TexCoordP2uiv(type_: GLenum, coords: *const GLuint) -> ();
    fn TexCoordP3ui(type_: GLenum, coords: GLuint) -> ();
    fn TexCoordP3uiv(type_: GLenum, coords: *const GLuint) -> ();
    fn TexCoordP4ui(type_: GLenum, coords: GLuint) -> ();
    fn TexCoordP4uiv(type_: GLenum, coords: *const GLuint) -> ();
    fn TexCoordPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> ();
    fn TexEnvf(target: GLenum, pname: GLenum, param: GLfloat) -> ();
    fn TexEnvfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> ();
    fn TexEnvi(target: GLenum, pname: GLenum, param: GLint) -> ();
    fn TexEnviv(target: GLenum, pname: GLenum, params: *const GLint) -> ();
    fn TexGend(coord: GLenum, pname: GLenum, param: GLdouble) -> ();
    fn TexGendv(coord: GLenum, pname: GLenum, params: *const GLdouble) -> ();
    fn TexGenf(coord: GLenum, pname: GLenum, param: GLfloat) -> ();
    fn TexGenfv(coord: GLenum, pname: GLenum, params: *const GLfloat) -> ();
    fn TexGeni(coord: GLenum, pname: GLenum, param: GLint) -> ();
    fn TexGeniv(coord: GLenum, pname: GLenum, params: *const GLint) -> ();
    fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> ();
    fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> ();
    fn TexImage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> ();
    fn TexImage3D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const c_void) -> (),
      or TexImage3DEXT;
    fn TexImage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> ();
    fn TexParameterIiv(target: GLenum, pname: GLenum, params: *const GLint) -> (),
      or TexParameterIivEXT,
      or TexParameterIivOES;
    fn TexParameterIuiv(target: GLenum, pname: GLenum, params: *const GLuint) -> (),
      or TexParameterIuivEXT,
      or TexParameterIuivOES;
    fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> ();
    fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> ();
    fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> ();
    fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> ();
    fn TexStorage1D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> (),
      or TexStorage1DEXT;
    fn TexStorage2D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> (),
      or TexStorage2DEXT;
    fn TexStorage2DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> ();
    fn TexStorage3D(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> (),
      or TexStorage3DEXT;
    fn TexStorage3DMultisample(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> (),
      or TexStorage3DMultisampleOES;
    fn TexSubImage1D(target: GLenum, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> (),
      or TexSubImage1DEXT;
    fn TexSubImage2D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> (),
      or TexSubImage2DEXT;
    fn TexSubImage3D(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> (),
      or TexSubImage3DEXT;
    fn TextureBarrier() -> ();
    fn TextureBuffer(texture: GLuint, internalformat: GLenum, buffer: GLuint) -> ();
    fn TextureBufferRange(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> ();
    fn TextureParameterIiv(texture: GLuint, pname: GLenum, params: *const GLint) -> ();
    fn TextureParameterIuiv(texture: GLuint, pname: GLenum, params: *const GLuint) -> ();
    fn TextureParameterf(texture: GLuint, pname: GLenum, param: GLfloat) -> ();
    fn TextureParameterfv(texture: GLuint, pname: GLenum, param: *const GLfloat) -> ();
    fn TextureParameteri(texture: GLuint, pname: GLenum, param: GLint) -> ();
    fn TextureParameteriv(texture: GLuint, pname: GLenum, param: *const GLint) -> ();
    fn TextureStorage1D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei) -> ();
    fn TextureStorage2D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) -> ();
    fn TextureStorage2DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean) -> ();
    fn TextureStorage3D(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) -> ();
    fn TextureStorage3DMultisample(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean) -> ();
    fn TextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> ();
    fn TextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> ();
    fn TextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const c_void) -> ();
    fn TextureView(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint) -> (),
      or TextureViewEXT,
      or TextureViewOES;
    fn TransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint) -> ();
    fn TransformFeedbackBufferRange(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr) -> ();
    fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum) -> (),
      or TransformFeedbackVaryingsEXT;
    fn Translated(x: GLdouble, y: GLdouble, z: GLdouble) -> ();
    fn Translatef(x: GLfloat, y: GLfloat, z: GLfloat) -> ();
    fn Uniform1d(location: GLint, x: GLdouble) -> ();
    fn Uniform1dv(location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn Uniform1f(location: GLint, v0: GLfloat) -> (),
      or Uniform1fARB;
    fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or Uniform1fvARB;
    fn Uniform1i(location: GLint, v0: GLint) -> (),
      or Uniform1iARB;
    fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
      or Uniform1ivARB;
    fn Uniform1ui(location: GLint, v0: GLuint) -> (),
      or Uniform1uiEXT;
    fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or Uniform1uivEXT;
    fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble) -> ();
    fn Uniform2dv(location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) -> (),
      or Uniform2fARB;
    fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or Uniform2fvARB;
    fn Uniform2i(location: GLint, v0: GLint, v1: GLint) -> (),
      or Uniform2iARB;
    fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
      or Uniform2ivARB;
    fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint) -> (),
      or Uniform2uiEXT;
    fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or Uniform2uivEXT;
    fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble) -> ();
    fn Uniform3dv(location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) -> (),
      or Uniform3fARB;
    fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or Uniform3fvARB;
    fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) -> (),
      or Uniform3iARB;
    fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
      or Uniform3ivARB;
    fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) -> (),
      or Uniform3uiEXT;
    fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or Uniform3uivEXT;
    fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> ();
    fn Uniform4dv(location: GLint, count: GLsizei, value: *const GLdouble) -> ();
    fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) -> (),
      or Uniform4fARB;
    fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat) -> (),
      or Uniform4fvARB;
    fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) -> (),
      or Uniform4iARB;
    fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint) -> (),
      or Uniform4ivARB;
    fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) -> (),
      or Uniform4uiEXT;
    fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint) -> (),
      or Uniform4uivEXT;
    fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint) -> ();
    fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix2fvARB;
    fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix2x3fvNV;
    fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix2x4fvNV;
    fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix3fvARB;
    fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix3x2fvNV;
    fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix3x4fvNV;
    fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix4fvARB;
    fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix4x2fvNV;
    fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble) -> ();
    fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat) -> (),
      or UniformMatrix4x3fvNV;
    fn UniformSubroutinesuiv(shadertype: GLenum, count: GLsizei, indices: *const GLuint) -> ();
    fn UnmapBuffer(target: GLenum) -> GLboolean,
      or UnmapBufferARB,
      or UnmapBufferOES;
    fn UnmapNamedBuffer(buffer: GLuint) -> GLboolean;
    fn UseProgram(program: GLuint) -> (),
      or UseProgramObjectARB;
    fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint) -> ();
    fn ValidateProgram(program: GLuint) -> (),
      or ValidateProgramARB;
    fn ValidateProgramPipeline(pipeline: GLuint) -> ();
    fn Vertex2d(x: GLdouble, y: GLdouble) -> ();
    fn Vertex2dv(v: *const GLdouble) -> ();
    fn Vertex2f(x: GLfloat, y: GLfloat) -> ();
    fn Vertex2fv(v: *const GLfloat) -> ();
    fn Vertex2i(x: GLint, y: GLint) -> ();
    fn Vertex2iv(v: *const GLint) -> ();
    fn Vertex2s(x: GLshort, y: GLshort) -> ();
    fn Vertex2sv(v: *const GLshort) -> ();
    fn Vertex3d(x: GLdouble, y: GLdouble, z: GLdouble) -> ();
    fn Vertex3dv(v: *const GLdouble) -> ();
    fn Vertex3f(x: GLfloat, y: GLfloat, z: GLfloat) -> ();
    fn Vertex3fv(v: *const GLfloat) -> ();
    fn Vertex3i(x: GLint, y: GLint, z: GLint) -> ();
    fn Vertex3iv(v: *const GLint) -> ();
    fn Vertex3s(x: GLshort, y: GLshort, z: GLshort) -> ();
    fn Vertex3sv(v: *const GLshort) -> ();
    fn Vertex4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> ();
    fn Vertex4dv(v: *const GLdouble) -> ();
    fn Vertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> ();
    fn Vertex4fv(v: *const GLfloat) -> ();
    fn Vertex4i(x: GLint, y: GLint, z: GLint, w: GLint) -> ();
    fn Vertex4iv(v: *const GLint) -> ();
    fn Vertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> ();
    fn Vertex4sv(v: *const GLshort) -> ();
    fn VertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint) -> ();
    fn VertexArrayAttribFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> ();
    fn VertexArrayAttribIFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> ();
    fn VertexArrayAttribLFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> ();
    fn VertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint) -> ();
    fn VertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint) -> ();
    fn VertexArrayVertexBuffer(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) -> ();
    fn VertexArrayVertexBuffers(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei) -> ();
    fn VertexAttrib1d(index: GLuint, x: GLdouble) -> (),
      or VertexAttrib1dARB,
      or VertexAttrib1dNV;
    fn VertexAttrib1dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttrib1dvARB,
      or VertexAttrib1dvNV;
    fn VertexAttrib1f(index: GLuint, x: GLfloat) -> (),
      or VertexAttrib1fARB,
      or VertexAttrib1fNV;
    fn VertexAttrib1fv(index: GLuint, v: *const GLfloat) -> (),
      or VertexAttrib1fvARB,
      or VertexAttrib1fvNV;
    fn VertexAttrib1s(index: GLuint, x: GLshort) -> (),
      or VertexAttrib1sARB,
      or VertexAttrib1sNV;
    fn VertexAttrib1sv(index: GLuint, v: *const GLshort) -> (),
      or VertexAttrib1svARB,
      or VertexAttrib1svNV;
    fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble) -> (),
      or VertexAttrib2dARB,
      or VertexAttrib2dNV;
    fn VertexAttrib2dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttrib2dvARB,
      or VertexAttrib2dvNV;
    fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat) -> (),
      or VertexAttrib2fARB,
      or VertexAttrib2fNV;
    fn VertexAttrib2fv(index: GLuint, v: *const GLfloat) -> (),
      or VertexAttrib2fvARB,
      or VertexAttrib2fvNV;
    fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort) -> (),
      or VertexAttrib2sARB,
      or VertexAttrib2sNV;
    fn VertexAttrib2sv(index: GLuint, v: *const GLshort) -> (),
      or VertexAttrib2svARB,
      or VertexAttrib2svNV;
    fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> (),
      or VertexAttrib3dARB,
      or VertexAttrib3dNV;
    fn VertexAttrib3dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttrib3dvARB,
      or VertexAttrib3dvNV;
    fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) -> (),
      or VertexAttrib3fARB,
      or VertexAttrib3fNV;
    fn VertexAttrib3fv(index: GLuint, v: *const GLfloat) -> (),
      or VertexAttrib3fvARB,
      or VertexAttrib3fvNV;
    fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort) -> (),
      or VertexAttrib3sARB,
      or VertexAttrib3sNV;
    fn VertexAttrib3sv(index: GLuint, v: *const GLshort) -> (),
      or VertexAttrib3svARB,
      or VertexAttrib3svNV;
    fn VertexAttrib4Nbv(index: GLuint, v: *const GLbyte) -> (),
      or VertexAttrib4NbvARB;
    fn VertexAttrib4Niv(index: GLuint, v: *const GLint) -> (),
      or VertexAttrib4NivARB;
    fn VertexAttrib4Nsv(index: GLuint, v: *const GLshort) -> (),
      or VertexAttrib4NsvARB;
    fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte) -> (),
      or VertexAttrib4NubARB,
      or VertexAttrib4ubNV;
    fn VertexAttrib4Nubv(index: GLuint, v: *const GLubyte) -> (),
      or VertexAttrib4NubvARB,
      or VertexAttrib4ubvNV;
    fn VertexAttrib4Nuiv(index: GLuint, v: *const GLuint) -> (),
      or VertexAttrib4NuivARB;
    fn VertexAttrib4Nusv(index: GLuint, v: *const GLushort) -> (),
      or VertexAttrib4NusvARB;
    fn VertexAttrib4bv(index: GLuint, v: *const GLbyte) -> (),
      or VertexAttrib4bvARB;
    fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> (),
      or VertexAttrib4dARB,
      or VertexAttrib4dNV;
    fn VertexAttrib4dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttrib4dvARB,
      or VertexAttrib4dvNV;
    fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> (),
      or VertexAttrib4fARB,
      or VertexAttrib4fNV;
    fn VertexAttrib4fv(index: GLuint, v: *const GLfloat) -> (),
      or VertexAttrib4fvARB,
      or VertexAttrib4fvNV;
    fn VertexAttrib4iv(index: GLuint, v: *const GLint) -> (),
      or VertexAttrib4ivARB;
    fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> (),
      or VertexAttrib4sARB,
      or VertexAttrib4sNV;
    fn VertexAttrib4sv(index: GLuint, v: *const GLshort) -> (),
      or VertexAttrib4svARB,
      or VertexAttrib4svNV;
    fn VertexAttrib4ubv(index: GLuint, v: *const GLubyte) -> (),
      or VertexAttrib4ubvARB;
    fn VertexAttrib4uiv(index: GLuint, v: *const GLuint) -> (),
      or VertexAttrib4uivARB;
    fn VertexAttrib4usv(index: GLuint, v: *const GLushort) -> (),
      or VertexAttrib4usvARB;
    fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint) -> ();
    fn VertexAttribDivisor(index: GLuint, divisor: GLuint) -> (),
      or VertexAttribDivisorANGLE,
      or VertexAttribDivisorARB,
      or VertexAttribDivisorEXT,
      or VertexAttribDivisorNV;
    fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint) -> ();
    fn VertexAttribI1i(index: GLuint, x: GLint) -> (),
      or VertexAttribI1iEXT;
    fn VertexAttribI1iv(index: GLuint, v: *const GLint) -> (),
      or VertexAttribI1ivEXT;
    fn VertexAttribI1ui(index: GLuint, x: GLuint) -> (),
      or VertexAttribI1uiEXT;
    fn VertexAttribI1uiv(index: GLuint, v: *const GLuint) -> (),
      or VertexAttribI1uivEXT;
    fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint) -> (),
      or VertexAttribI2iEXT;
    fn VertexAttribI2iv(index: GLuint, v: *const GLint) -> (),
      or VertexAttribI2ivEXT;
    fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint) -> (),
      or VertexAttribI2uiEXT;
    fn VertexAttribI2uiv(index: GLuint, v: *const GLuint) -> (),
      or VertexAttribI2uivEXT;
    fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint) -> (),
      or VertexAttribI3iEXT;
    fn VertexAttribI3iv(index: GLuint, v: *const GLint) -> (),
      or VertexAttribI3ivEXT;
    fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint) -> (),
      or VertexAttribI3uiEXT;
    fn VertexAttribI3uiv(index: GLuint, v: *const GLuint) -> (),
      or VertexAttribI3uivEXT;
    fn VertexAttribI4bv(index: GLuint, v: *const GLbyte) -> (),
      or VertexAttribI4bvEXT;
    fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) -> (),
      or VertexAttribI4iEXT;
    fn VertexAttribI4iv(index: GLuint, v: *const GLint) -> (),
      or VertexAttribI4ivEXT;
    fn VertexAttribI4sv(index: GLuint, v: *const GLshort) -> (),
      or VertexAttribI4svEXT;
    fn VertexAttribI4ubv(index: GLuint, v: *const GLubyte) -> (),
      or VertexAttribI4ubvEXT;
    fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) -> (),
      or VertexAttribI4uiEXT;
    fn VertexAttribI4uiv(index: GLuint, v: *const GLuint) -> (),
      or VertexAttribI4uivEXT;
    fn VertexAttribI4usv(index: GLuint, v: *const GLushort) -> (),
      or VertexAttribI4usvEXT;
    fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> ();
    fn VertexAttribIPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> (),
      or VertexAttribIPointerEXT;
    fn VertexAttribL1d(index: GLuint, x: GLdouble) -> (),
      or VertexAttribL1dEXT;
    fn VertexAttribL1dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttribL1dvEXT;
    fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble) -> (),
      or VertexAttribL2dEXT;
    fn VertexAttribL2dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttribL2dvEXT;
    fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble) -> (),
      or VertexAttribL3dEXT;
    fn VertexAttribL3dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttribL3dvEXT;
    fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> (),
      or VertexAttribL4dEXT;
    fn VertexAttribL4dv(index: GLuint, v: *const GLdouble) -> (),
      or VertexAttribL4dvEXT;
    fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint) -> ();
    fn VertexAttribLPointer(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> (),
      or VertexAttribLPointerEXT;
    fn VertexAttribP1ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> ();
    fn VertexAttribP1uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> ();
    fn VertexAttribP2ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> ();
    fn VertexAttribP2uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> ();
    fn VertexAttribP3ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> ();
    fn VertexAttribP3uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> ();
    fn VertexAttribP4ui(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint) -> ();
    fn VertexAttribP4uiv(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint) -> ();
    fn VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void) -> (),
      or VertexAttribPointerARB;
    fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint) -> ();
    fn VertexP2ui(type_: GLenum, value: GLuint) -> ();
    fn VertexP2uiv(type_: GLenum, value: *const GLuint) -> ();
    fn VertexP3ui(type_: GLenum, value: GLuint) -> ();
    fn VertexP3uiv(type_: GLenum, value: *const GLuint) -> ();
    fn VertexP4ui(type_: GLenum, value: GLuint) -> ();
    fn VertexP4uiv(type_: GLenum, value: *const GLuint) -> ();
    fn VertexPointer(size: GLint, type_: GLenum, stride: GLsizei, pointer: *const c_void) -> ();
    fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> ();
    fn ViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat) -> (),
      or ViewportArrayvNV,
      or ViewportArrayvOES;
    fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat) -> (),
      or ViewportIndexedfOES,
      or ViewportIndexedfNV;
    fn ViewportIndexedfv(index: GLuint, v: *const GLfloat) -> (),
      or ViewportIndexedfvOES,
      or ViewportIndexedfvNV;
    fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> (),
      or WaitSyncAPPLE;
    fn WindowPos2d(x: GLdouble, y: GLdouble) -> (),
      or WindowPos2dARB,
      or WindowPos2dMESA;
    fn WindowPos2dv(v: *const GLdouble) -> (),
      or WindowPos2dvARB,
      or WindowPos2dvMESA;
    fn WindowPos2f(x: GLfloat, y: GLfloat) -> (),
      or WindowPos2fARB,
      or WindowPos2fMESA;
    fn WindowPos2fv(v: *const GLfloat) -> (),
      or WindowPos2fvARB,
      or WindowPos2fvMESA;
    fn WindowPos2i(x: GLint, y: GLint) -> (),
      or WindowPos2iARB,
      or WindowPos2iMESA;
    fn WindowPos2iv(v: *const GLint) -> (),
      or WindowPos2ivARB,
      or WindowPos2ivMESA;
    fn WindowPos2s(x: GLshort, y: GLshort) -> (),
      or WindowPos2sARB,
      or WindowPos2sMESA;
    fn WindowPos2sv(v: *const GLshort) -> (),
      or WindowPos2svARB,
      or WindowPos2svMESA;
    fn WindowPos3d(x: GLdouble, y: GLdouble, z: GLdouble) -> (),
      or WindowPos3dARB,
      or WindowPos3dMESA;
    fn WindowPos3dv(v: *const GLdouble) -> (),
      or WindowPos3dvARB,
      or WindowPos3dvMESA;
    fn WindowPos3f(x: GLfloat, y: GLfloat, z: GLfloat) -> (),
      or WindowPos3fARB,
      or WindowPos3fMESA;
    fn WindowPos3fv(v: *const GLfloat) -> (),
      or WindowPos3fvARB,
      or WindowPos3fvMESA;
    fn WindowPos3i(x: GLint, y: GLint, z: GLint) -> (),
      or WindowPos3iARB,
      or WindowPos3iMESA;
    fn WindowPos3iv(v: *const GLint) -> (),
      or WindowPos3ivARB,
      or WindowPos3ivMESA;
    fn WindowPos3s(x: GLshort, y: GLshort, z: GLshort) -> (),
      or WindowPos3sARB,
      or WindowPos3sMESA;
    fn WindowPos3sv(v: *const GLshort) -> (),
      or WindowPos3svARB,
      or WindowPos3svMESA;
}

#[allow(missing_copy_implementations)]
pub struct FnPtr {
    /// The function pointer that will be used when calling the function.
    f: *const c_void,
    /// True if the pointer points to a real function, false if points to a `panic!` fn.
    is_loaded: bool,
}

/// Load each OpenGL symbol using a custom load function. This allows for the
/// use of functions like `glfwGetProcAddress` @`SDL_GL_GetProcAddress`.
/// ~~~ignore
/// gl::load_with(|s| glfw.get_proc_address(s));
/// ~~~
#[allow(dead_code)]
pub fn load_with<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const c_void {
    inner_load(&mut loadfn)
}
    
