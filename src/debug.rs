use core::cmp::{Eq, PartialEq};
use std::ffi::{c_void, CStr};
use super::gl;
use super::raw::{GLenum, GLuint, GLsizei, GLchar};

pub macro gl_enum(
    $(#[$attr:meta])*
    $vis:vis enum $name:ident($ty:ty) {
        $($variant:ident = $value:path),*
    }
) {
    $(#[$attr])*
    #[repr($ty)]
    $vis enum $name {
        $($variant = $value),*,
        Unknown($ty)
    }

    impl From<$ty> for $name {
        #[inline(always)]
        fn from(value: $ty) -> Self {
            match value {
                $($value => $name::$variant),*,
                other => $name::Unknown(other)
            }
        }
    }
}

gl_enum! {
    #[derive(Debug)]
    pub enum DebugSource(u32) {
        Api            = gl::DEBUG_SOURCE_API,
        WindowSystem   = gl::DEBUG_SOURCE_WINDOW_SYSTEM,
        ShaderCompiler = gl::DEBUG_SOURCE_SHADER_COMPILER,
        ThirdParty     = gl::DEBUG_SOURCE_THIRD_PARTY,
        Application    = gl::DEBUG_SOURCE_APPLICATION,
        Other          = gl::DEBUG_SOURCE_OTHER
    }
}

gl_enum! {
    #[derive(Debug)]
    pub enum DebugType(u32) {
        Error              = gl::DEBUG_TYPE_ERROR,
        DeprecatedBehavior = gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR,
        UndefinedBehavior  = gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR,
        Portability        = gl::DEBUG_TYPE_PORTABILITY,
        Performance        = gl::DEBUG_TYPE_PERFORMANCE,
        Marker             = gl::DEBUG_TYPE_MARKER,
        PushGroup          = gl::DEBUG_TYPE_PUSH_GROUP,
        PopGroup           = gl::DEBUG_TYPE_POP_GROUP,
        Other              = gl::DEBUG_TYPE_OTHER
    }
}

gl_enum! {
    #[derive(Debug)]
    pub enum DebugSeverity(u32) {
        High         = gl::DEBUG_SEVERITY_HIGH,
        Medium       = gl::DEBUG_SEVERITY_MEDIUM,
        Low          = gl::DEBUG_SEVERITY_LOW,
        Notification = gl::DEBUG_SEVERITY_NOTIFICATION
    }
}

gl_enum! {
    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
    pub enum GlError(u32) {
        NoError                     = gl::NO_ERROR,
        InvalidEnum                 = gl::INVALID_ENUM,
        InvalidValue                = gl::INVALID_VALUE,
        InvalidOperation            = gl::INVALID_OPERATION,
        StackOverflow               = gl::STACK_OVERFLOW,
        StackUnderflow              = gl::STACK_UNDERFLOW,
        OutOfMemory                 = gl::OUT_OF_MEMORY,
        InvalidFramebufferOperation = gl::INVALID_FRAMEBUFFER_OPERATION,
        ContextLost                 = gl::CONTEXT_LOST,
        TableTooLarge               = gl::TABLE_TOO_LARGE
    }
}

impl GlError {
    #[inline(always)]
    pub fn get() -> Self {
        Self::from(unsafe { gl::GetError() })
    }

    pub fn to_result(self) -> Result<(), GlError> {
        match self {
            GlError::NoError => Ok(()),
            other => Err(other),
        }
    }
}

type DebugCallback<'a> = dyn FnMut(DebugSource, DebugSeverity, DebugType, GlError, &'a CStr);

pub fn enable<C>(callback: C) where C: FnMut(DebugSource, DebugSeverity, DebugType, GlError, &CStr) + 'static {
    extern "system" fn debug_callback(source: GLenum,
                                      gltype: GLenum,
                                      id: GLuint,
                                      severity: GLenum,
                                      length: GLsizei,
                                      message: *const GLchar,
                                      callback: *mut c_void) {

        unsafe {
            let mut callback = callback.cast::<Box<DebugCallback>>();
            if let Some(callback) = callback.as_mut() {
                let source = DebugSource::from(source);
                let ty = DebugType::from(gltype);
                let severity = DebugSeverity::from(severity);
                let id = GlError::from(id);

                let message: &[u8] = std::slice::from_raw_parts(message.cast(), length as _);
                let message = CStr::from_bytes_with_nul_unchecked(message);

                callback(source, severity, ty, id, message);
            }
        }
    }

    let callback: Box<Box<DebugCallback>> = Box::new(Box::new(callback));

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(debug_callback), Box::into_raw(callback).cast());
    }
}