use std::num::NonZeroU32;
use std::ops::Range;
use crate::gl;
use crate::raw::types::GLenum;
use crate::state::GraphicsContext;

pub struct DisplayListIter {
    pub(crate) range: Range<NonZeroU32>,
    pub(crate) context: GraphicsContext
}

impl Iterator for DisplayListIter {
    type Item = DisplayList;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.range.start;
        if id >= self.range.end {
            None
        } else {
            self.range.start = self.range.start.saturating_add(1);
            Some(DisplayList(id, self.context))
        }
    }
}

impl Drop for DisplayListIter {
    fn drop(&mut self) {
        unsafe {
            let Range { start, end} = self.range;
            gl::DeleteLists(start.get(), (end.get() - start.get()) as _);
        }
    }
}

pub struct DisplayList(pub NonZeroU32, pub(crate) GraphicsContext);

impl DisplayList {
    pub fn compile<C>(&self, call: C) where C: FnOnce(&GraphicsContext) -> () {
        self.compile0(call, gl::COMPILE)
    }

    pub fn compile_and_exec<C>(&self, call: C) where C: FnOnce(&GraphicsContext) -> () {
        self.compile0(call, gl::COMPILE_AND_EXECUTE)
    }

    #[inline(always)]
    fn compile0<C>(&self, call: C, mode: GLenum) where C: FnOnce(&GraphicsContext) -> () {
        unsafe {
            gl::NewList(self.0.get(), mode);
        }
        (call)(&self.1);
        unsafe {
            gl::EndList();
        }
    }
}

impl Drop for DisplayList {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteLists(self.0.get(), 1);
        }
    }
}