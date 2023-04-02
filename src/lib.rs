#![feature(decl_macro)]
#![feature(adt_const_params)]

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};


pub use cgmath as math;
pub use image;
use crate::state::GraphicsContext;

pub(crate) mod gl;
pub(crate) mod raw;
pub mod debug;
pub mod state;
pub mod framebuffer;
pub mod buffer;
pub mod texture;
pub mod matrix;
pub mod tessellator;

static LOADED: AtomicBool = AtomicBool::new(false);

pub fn load_with<F>(loader: F) -> GraphicsContext where F: FnMut(&'static str) -> *const c_void {
    LOADED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .expect("Already loaded");

    gl::load_with(loader);

    unsafe {
        GraphicsContext::new()
    }
}
