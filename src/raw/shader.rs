use std::ffi::CStr;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::ptr::null_mut;
use cgmath::{vec3, Matrix, Matrix2, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use crate::debug::GlError;
use crate::gl;
use crate::gl::GLint;
use crate::shader::ProgramError;
use crate::state::GraphicsContext;
use crate::tessellator::Vertex;

#[repr(transparent)]
pub struct ShaderId(pub NonZeroU32, pub(crate) GraphicsContext);

#[repr(transparent)]
#[derive(Debug)]
pub struct BinaryFormat(pub u32);

#[derive(Debug)]
pub struct ProgramBinary {
    pub format: BinaryFormat,
    pub binary: Vec<u8>
}

#[repr(u32)]
enum ShaderParam {
    Type          = gl::SHADER_TYPE,
    DeleteStatus  = gl::DELETE_STATUS,
    CompileStatus = gl::COMPILE_STATUS,
    InfoLogLength = gl::INFO_LOG_LENGTH,
    SourceLength  = gl::SHADER_SOURCE_LENGTH,
}

#[repr(u32)]
enum ProgramParam {
    DeleteStatus                      = gl::DELETE_STATUS,
    LinkStatus                        = gl::LINK_STATUS,
    ValidateStatus                    = gl::VALIDATE_STATUS,
    InfoLogLength                     = gl::INFO_LOG_LENGTH,
    AttachedShaders                   = gl::ATTACHED_SHADERS,
    ActiveAtomicCounterBuffers        = gl::ACTIVE_ATOMIC_COUNTER_BUFFERS,
    ActiveAttributes                  = gl::ACTIVE_ATTRIBUTES,
    ActiveAttributeMaxLength          = gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
    ActiveUniforms                    = gl::ACTIVE_UNIFORMS,
    ActiveUniformBlocks               = gl::ACTIVE_UNIFORM_BLOCKS,
    ActiveUniformBlockMaxNameLength   = gl::ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,
    ActiveUniformMaxLength            = gl::ACTIVE_UNIFORM_MAX_LENGTH,
    ComputeWorkGroupSize              = gl::COMPUTE_WORK_GROUP_SIZE,
    BinaryLength                      = gl::PROGRAM_BINARY_LENGTH,
    TransformFeedbackBufferMode       = gl::TRANSFORM_FEEDBACK_BUFFER_MODE,
    TransformFeedbackVaryings         = gl::TRANSFORM_FEEDBACK_VARYINGS,
    TransformFeedbackVaryingMaxLength = gl::TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,
    GeometryVerticesOut               = gl::GEOMETRY_VERTICES_OUT,
    GeometryInputType                 = gl::GEOMETRY_INPUT_TYPE,
    GeometryOutputType                = gl::GEOMETRY_OUTPUT_TYPE,
}

#[repr(u32)]
pub enum ShaderType {
    Compute        = gl::COMPUTE_SHADER,
    Vertex         = gl::VERTEX_SHADER,
    TessControl    = gl::TESS_CONTROL_SHADER,
    TessEvaluation = gl::TESS_EVALUATION_SHADER,
    Geometry       = gl::GEOMETRY_SHADER,
    Fragment       = gl::FRAGMENT_SHADER,
}

#[derive(Debug)]
pub struct ActiveUniform<'a> {
    pub id: u32,
    pub name: &'a str,
    pub size: u32,
    pub ty: u32
}

#[derive(Debug)]
pub struct ActiveAttribute<'a> {
    pub id: u32,
    pub name: &'a str,
    pub size: u32,
    pub ty: u32
}

impl GraphicsContext {
    pub fn create_shader(&self, ty: ShaderType) -> Result<ShaderId, GlError> {
        unsafe {
            let id = gl::CreateShader(ty as _);
            NonZeroU32::new(id).map(move |i| ShaderId(i, *self)).ok_or_else(GlError::get)
        }
    }

    pub fn create_program(&self) -> Result<ProgramId, GlError> {
        unsafe {
            let id = gl::CreateProgram();
            NonZeroU32::new(id).map(move |i| ProgramId(i, *self)).ok_or_else(GlError::get)
        }
    }

    pub fn shader_binary(&self, shaders: &[u32], format: BinaryFormat, binary: &[u8]) -> Result<(), GlError> {
        unsafe {
            gl::ShaderBinary(shaders.len() as _, shaders.as_ptr(), format.0, binary.as_ptr().cast(), binary.len() as _);
            GlError::get().to_result()
        }
    }
}

impl ShaderId {
    #[inline(always)]
    pub fn id(&self) -> u32 {
        self.0.get()
    }

    pub fn binary(&self, format: BinaryFormat, binary: &[u8]) -> Result<(), GlError> {
        let id = self.id();
        self.1.shader_binary(&[id], format, binary)
    }

    pub fn source(&self, source: &str) -> Result<(), GlError> {
        unsafe {
            let lengths = [source.len()];
            gl::ShaderSource(self.id(), 1, &source.as_ptr().cast(), lengths.as_ptr().cast());
            GlError::get().to_result()
        }
    }

    pub fn get_source(&self) -> Result<String, GlError> {
        let len = self.get(ShaderParam::SourceLength);
        let mut source = Vec::with_capacity(len as _);
        unsafe {
            gl::GetShaderSource(self.id(), len, null_mut(), source.as_mut_ptr());
            source.set_len((len - 1).max(0) as _);
            GlError::get().to_result().map(|_| String::from_utf8_unchecked(std::mem::transmute(source)))
        }
    }

    pub fn compile(&self) -> Result<(), String> {
        unsafe {
            gl::CompileShader(self.id());
        }
        let status = self.get(ShaderParam::CompileStatus);
        if status == gl::TRUE as _ {
            Ok(())
        } else {
            Err(self.get_info_log())
        }
    }

    pub fn get_info_log(&self) -> String {
        let len = self.get(ShaderParam::InfoLogLength);
        let mut log = Vec::with_capacity(len as _);
        unsafe {
            gl::GetShaderInfoLog(self.id(), len, null_mut(), log.as_mut_ptr());
            log.set_len((len - 1).max(0) as _);
            String::from_utf8_unchecked(std::mem::transmute(log))
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            gl::IsShader(self.id()) == gl::TRUE
        }
    }

    fn get(&self, param: ShaderParam) -> i32 {
        unsafe {
            let mut result = 0;
            gl::GetShaderiv(self.id(), param as _, &mut result);
            result
        }
    }
}

impl Drop for ShaderId {
    #[inline(always)]
    fn drop(&mut self) {
        let id = self.id();
        unsafe {
            gl::DeleteShader(id)
        }
    }
}

#[repr(transparent)]
pub struct ProgramId(pub NonZeroU32, pub(crate) GraphicsContext);

#[repr(transparent)]
pub struct LinkedProgramId<V>(ProgramId, PhantomData<V>);

impl<V> Deref for LinkedProgramId<V> {
    type Target = ProgramId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for ProgramId {
    #[inline(always)]
    fn drop(&mut self) {
        let id = self.id();
        unsafe {
            gl::DeleteProgram(id)
        }
    }
}

impl<V: Vertex> LinkedProgramId<V> {
    pub fn uniform<'a, 'b, U>(&'a self, name: &'b CStr) -> Option<UniformLocation<'a, 'b, U, V>> where U: UniformValue {
        unsafe {
            let location = gl::GetUniformLocation(self.id(), name.as_ptr());
            if location == -1 {
                return None;
            }
            Some(UniformLocation {
                program: self,
                location,
                name,
                _ty: PhantomData
            })
        }
    }

    pub fn attribute<'a, 'b, A>(&'a self, name: &'b CStr) -> Option<AttributeLocation<'a, 'b, A, V>> {
        unsafe {
            let location = gl::GetAttribLocation(self.id(), name.as_ptr());
            if location == -1 {
                return None;
            }
            Some(AttributeLocation {
                program: self,
                location,
                name,
                _ty: PhantomData
            })
        }
    }

    pub fn validate(&self) -> Result<String, GlError> {
        unsafe {
            gl::ValidateProgram(self.id());
            let status = self.get(ProgramParam::ValidateStatus);
            if status == gl::TRUE as _ {
                Ok(self.get_info_log())
            } else {
                Err(GlError::get())
            }
        }
    }

    pub fn get_active_uniforms<'b, 'p>(&'p self, buffer: &'b mut [u8]) -> ActiveUniforms<'b, 'p, V> {
        let count = self.get(ProgramParam::ActiveUniforms) as u32;
        ActiveUniforms {
            program: self,
            buffer,
            index: 0,
            count,
        }
    }

    pub fn get_active_attributes<'b, 'p>(&'p self, buffer: &'b mut [u8]) -> ActiveAttributes<'b, 'p, V> {
        let count = self.get(ProgramParam::ActiveAttributes) as u32;
        ActiveAttributes {
            program: self,
            buffer,
            index: 0,
            count,
        }
    }
}

pub trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;
}

pub struct ActiveAttributes<'buffer, 'program, V> {
    program: &'program LinkedProgramId<V>,
    buffer: &'buffer mut [u8],
    index: u32,
    count: u32
}

impl<'buffer, 'program, V> LendingIterator for ActiveAttributes<'buffer, 'program, V> {
    type Item<'a> = Result<ActiveAttribute<'a>, GlError> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.index < self.count {
            let max_len = self.buffer.len() - 1;
            let result = unsafe {
                let mut len = 0;
                let mut size = 0;
                let mut ty = 0;
                gl::GetActiveAttrib(self.program.id(), self.index as _, max_len as _, &mut len, &mut size, &mut ty, self.buffer.as_mut_ptr().cast());
                if len != 0 {
                    Ok(ActiveAttribute {
                        id: self.index,
                        name: std::str::from_utf8_unchecked(&self.buffer[0..len as usize]),
                        size: size as _,
                        ty: ty as _,
                    })
                } else {
                    Err(GlError::get())
                }
            };
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

pub struct ActiveUniforms<'buffer, 'program, V> {
    program: &'program LinkedProgramId<V>,
    buffer: &'buffer mut [u8],
    index: u32,
    count: u32
}

impl<'buffer, 'program, V> LendingIterator for ActiveUniforms<'buffer, 'program, V> {
    type Item<'a> = Result<ActiveUniform<'a>, GlError> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.index < self.count {
            let max_len = self.buffer.len() - 1;
            let result = unsafe {
                let mut len = 0;
                let mut size = 0;
                let mut ty = 0;
                gl::GetActiveUniform(self.program.id(), self.index as _, max_len as _, &mut len, &mut size, &mut ty, self.buffer.as_mut_ptr().cast());
                if len != 0 {
                    Ok(ActiveUniform {
                        id: self.index,
                        name: std::str::from_utf8_unchecked(&self.buffer[0..len as usize]),
                        size: size as _,
                        ty: ty as _,
                    })
                } else {
                    Err(GlError::get())
                }
            };
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl ProgramId {
    #[inline(always)]
    pub fn id(&self) -> u32 {
        self.0.get()
    }

    pub fn attach(&self, shader: &ShaderId) -> Result<(), GlError> {
        unsafe {
            gl::AttachShader(self.id(), shader.0.get());
            GlError::get().to_result()
        }
    }

    pub fn detach(&self, shader: &ShaderId) -> Result<(), GlError> {
        unsafe {
            gl::DetachShader(self.id(), shader.0.get());
            GlError::get().to_result()
        }
    }

    pub fn link<V>(self) -> Result<LinkedProgramId<V>, String> where V: Vertex {
        unsafe {
            gl::LinkProgram(self.id());
            let status = self.get(ProgramParam::LinkStatus);
            if status == gl::TRUE as _ {
                let linked = LinkedProgramId(self, PhantomData);
                Ok(linked)
            } else {
                Err(self.get_info_log())
            }
        }
    }

    pub fn binary(&self, binary: ProgramBinary)  -> Result<(), GlError> {
        unsafe {
            gl::ProgramBinary(self.id(), binary.format.0, binary.binary.as_ptr().cast(), binary.binary.len() as _);
            GlError::get().to_result()
        }
    }

    pub fn get_binary(&self) -> Result<ProgramBinary, GlError> {
        let len = self.get(ProgramParam::BinaryLength);
        let mut binary = Vec::<u8>::with_capacity(len as _);
        let mut format = 0;
        unsafe {
            gl::GetProgramBinary(self.id(), len, null_mut(), &mut format, binary.as_mut_ptr().cast());
            binary.set_len(len as _);
            GlError::get().to_result().map(|_| ProgramBinary {
                format: BinaryFormat(format),
                binary
            })
        }
    }

    pub fn get_info_log(&self) -> String {
        let len = self.get(ProgramParam::InfoLogLength);
        let mut log = Vec::<u8>::with_capacity(len as _);
        unsafe {
            gl::GetProgramInfoLog(self.id(), len, null_mut(), log.as_mut_ptr().cast());
            log.set_len((len - 1).max(0) as _);
            String::from_utf8_unchecked(log)
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            gl::IsProgram(self.id()) == gl::TRUE
        }
    }

    fn get(&self, param: ProgramParam) -> i32 {
        unsafe {
            let mut result = 0;
            gl::GetProgramiv(self.id(), param as _, &mut result);
            result
        }
    }
}

pub trait Program {
    type Format: Vertex;

    fn from_source(context: GraphicsContext, vertex: &str, fragment: &str) -> Result<Self, ProgramError> where Self: Sized;
}

pub struct AttributeLocation<'a, 'b, A, V> {
    program: &'a LinkedProgramId<V>,
    location: GLint,
    name: &'b CStr,
    _ty: PhantomData<A>
}

impl<'a, 'b, A, V> AttributeLocation<'a, 'b, A, V> {
    pub unsafe fn bind(&self, id: u32) {
        gl::BindAttribLocation(self.program.id(), id, self.name.as_ptr() as _);
    }
}

pub struct UniformLocation<'a, 'b, U, A> {
    program: &'a LinkedProgramId<A>,
    location: GLint,
    name: &'b CStr,
    _ty: PhantomData<U>
}

impl<'a, 'b, U, V> UniformLocation<'a, 'b, U, V> where U: UniformValue {
    pub fn set(&self, value: U) {
        U::set(value, self.location);
    }

    pub unsafe fn into_inner(self) -> GLint {
        self.location
    }
}

pub trait UniformValue {
    fn set(self, location: GLint);
}

macro_rules! v {
    ($ty: ty: $glfn1: ident, $glfn2: ident, $glfn3: ident, $glfn4: ident) => {
        impl UniformValue for $ty {
            fn set(self, location: GLint) {
                unsafe {
                    gl::$glfn1(location, self);
                }
            }
        }

        impl UniformValue for ($ty, $ty) {
            fn set(self, location: GLint) {
                unsafe {
                    let (x, y) = self;
                    gl::$glfn2(location, x, y);
                }
            }
        }

        impl UniformValue for [$ty; 2] {
            fn set(self, location: GLint) {
                unsafe {
                    let [x, y] = self;
                    gl::$glfn2(location, x, y);
                }
            }
        }

        impl UniformValue for Vector2<$ty> {
            fn set(self, location: GLint) {
                unsafe {
                    gl::$glfn2(location, self.x, self.y);
                }
            }
        }

        impl UniformValue for ($ty, $ty, $ty) {
            fn set(self, location: GLint) {
                unsafe {
                    let (x, y, z) = self;
                    gl::$glfn3(location, x, y, z);
                }
            }
        }

        impl UniformValue for [$ty; 3] {
            fn set(self, location: GLint) {
                unsafe {
                    let [x, y, z] = self;
                    gl::$glfn3(location, x, y, z);
                }
            }
        }

        impl UniformValue for Vector3<$ty> {
            fn set(self, location: GLint) {
                unsafe {
                    gl::$glfn3(location, self.x, self.y, self.z);
                }
            }
        }

        impl UniformValue for ($ty, $ty, $ty, $ty) {
            fn set(self, location: GLint) {
                unsafe {
                    let (x, y, z, w) = self;
                    gl::$glfn4(location, x, y, z, w);
                }
            }
        }

        impl UniformValue for [$ty; 4] {
            fn set(self, location: GLint) {
                unsafe {
                    let [x, y, z, w] = self;
                    gl::$glfn4(location, x, y, z, w);
                }
            }
        }

        impl UniformValue for Vector4<$ty> {
            fn set(self, location: GLint) {
                unsafe {
                    gl::$glfn4(location, self.x, self.y, self.z, self.w);
                }
            }
        }

        impl UniformValue for &[$ty] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn1, v))(location, self.len() as _, self.as_ptr());
                }
            }
        }

        impl UniformValue for &[[$ty; 2]] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn2, v))(location, self.len() as _, self.as_ptr().cast());
                }
            }
        }

        impl UniformValue for &[Vector2<$ty>] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn2, v))(location, self.len() as _, self.as_ptr().cast());
                }
            }
        }

        impl UniformValue for &[($ty, $ty)] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn2, v))(location, self.len() as _, self.as_ptr().cast());
                }
            }
        }

        impl UniformValue for &[[$ty; 3]] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn3, v))(location, self.len() as _, self.as_ptr().cast());
                }
            }
        }

        impl UniformValue for &[($ty, $ty, $ty)] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn3, v))(location, self.len() as _, self.as_ptr().cast());
                }
            }
        }

        impl UniformValue for &[[$ty; 4]] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn4, v))(location, self.len() as _, self.as_ptr().cast());
                }
            }
        }

        impl UniformValue for &[($ty, $ty, $ty, $ty)] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    (concat_idents!($glfn4, v))(location, self.len() as _, self.as_ptr().cast());
                }
            }
        }
    };
}

v!(f64: Uniform1d, Uniform2d, Uniform3d, Uniform4d);
v!(f32: Uniform1f, Uniform2f, Uniform3f, Uniform4f);
v!(i32: Uniform1i, Uniform2i, Uniform3i, Uniform4i);
v!(u32: Uniform1ui, Uniform2ui, Uniform3ui, Uniform4ui);

macro_rules! m {
    ($ty: ty: $glfn2: ident, $glfn3: ident, $glfn4: ident) => {
        impl UniformValue for Matrix2<$ty> {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn2(location, 1, 0, self.as_ptr())
                }
            }
        }

        impl UniformValue for &[[$ty; 2]; 2] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn2(location, 1, 0, self.as_ptr().cast())
                }
            }
        }

        impl UniformValue for &[$ty; 4] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn2(location, 1, 0, self.as_ptr())
                }
            }
        }

        impl UniformValue for &[Matrix2<$ty>] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn2(location, self.len() as _, 0, self.as_ptr().cast())
                }
            }
        }

        impl UniformValue for Matrix3<$ty> {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn3(location, 1, 0, self.as_ptr())
                }
            }
        }

        impl UniformValue for &[[$ty; 3]; 3] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn3(location, 1, 0, self.as_ptr().cast())
                }
            }
        }

        impl UniformValue for &[$ty; 9] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn3(location, 1, 0, self.as_ptr())
                }
            }
        }

        impl UniformValue for &[Matrix3<$ty>] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn3(location, self.len() as _, 0, self.as_ptr().cast())
                }
            }
        }

        impl UniformValue for &[[$ty; 9]] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn3(location, self.len() as _, 0, self.as_ptr().cast())
                }
            }
        }

        impl UniformValue for Matrix4<$ty> {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn4(location, 1, 0, self.as_ptr())
                }
            }
        }

        impl UniformValue for &[[$ty; 4]; 4] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn4(location, 1, 0, self.as_ptr().cast())
                }
            }
        }

        impl UniformValue for &[$ty; 16] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn4(location, 1, 0, self.as_ptr())
                }
            }
        }

        impl UniformValue for &[Matrix4<$ty>] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn4(location, self.len() as _, 0, self.as_ptr().cast())
                }
            }
        }

        impl UniformValue for &[[$ty; 16]] {
            fn set(self, location: GLint) {
                unsafe {
                    use gl::*;
                    $glfn4(location, self.len() as _, 0, self.as_ptr().cast())
                }
            }
        }
    };
}

m!(f32: UniformMatrix2fv, UniformMatrix3fv, UniformMatrix4fv);
m!(f64: UniformMatrix2dv, UniformMatrix3dv, UniformMatrix4dv);