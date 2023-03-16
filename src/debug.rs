use core::cmp::{Eq, PartialEq};
use std::ffi::{c_void, CStr};
use super::gl;
use super::raw::types::{GLenum, GLuint, GLsizei, GLchar};

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
}

pub fn enable() {
    extern "system" fn debug_callback(source: GLenum,
                                      gltype: GLenum,
                                      id: GLuint,
                                      severity: GLenum,
                                      length: GLsizei,
                                      message: *const GLchar,
                                      _user_param: *mut c_void) {

        unsafe {
            let source = DebugSource::from(source);
            let ty = DebugType::from(gltype);
            let severity = DebugSeverity::from(severity);
            let id = GlError::from(id);

            let message: &[u8] = std::slice::from_raw_parts(message.cast(), length as _);
            let message = CStr::from_bytes_with_nul_unchecked(message);
            eprintln!("{:?} [{:?}] {:?}: {:?} ({:?})", source, severity, ty, id, message);
        }
    }

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(debug_callback), std::ptr::null_mut());
    }
}