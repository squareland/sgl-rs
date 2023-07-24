use std::num::NonZeroU32;
use crate::gl;
use crate::state::GraphicsContext;

#[repr(transparent)]
pub struct QueryId(pub NonZeroU32, pub(crate) GraphicsContext);

impl Drop for QueryId {
    #[inline(always)]
    fn drop(&mut self) {
        let id = self.0.get();
        unsafe {
            gl::DeleteQueries(1, &id)
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[repr(u32)]
pub enum QueryTarget {
    SamplesPassed                      = gl::SAMPLES_PASSED,
    PrimitivesGenerated                = gl::PRIMITIVES_GENERATED,
    TransformFeedbackPrimitivesWritten = gl::TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN,
    TimeElapsed                        = gl::TIME_ELAPSED,
    Timestamp                          = gl::TIMESTAMP,
    AnySamplesPassed                   = gl::ANY_SAMPLES_PASSED,
    AnySamplesPassedConservative       = gl::ANY_SAMPLES_PASSED_CONSERVATIVE
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[repr(u32)]
pub enum QueryObjectParam {
    Result          = gl::QUERY_RESULT,
    ResultAvailable = gl::QUERY_RESULT_AVAILABLE,
    ResultNoWait    = gl::QUERY_RESULT_NO_WAIT,
    Target          = gl::QUERY_TARGET
}

impl QueryId {
    pub fn query<C>(&self, target: QueryTarget, call: C) where C: FnOnce(&GraphicsContext) -> () {
        unsafe {
            gl::BeginQuery(target as u32, self.0.get());
        }
        (call)(&self.1);
        unsafe {
            gl::EndQuery(target as u32);
        }
    }

    pub fn get_result_i32(&self) -> Option<i32> {
        let mut available = 0;
        unsafe {
            gl::GetQueryObjectuiv(self.0.get(), gl::QUERY_RESULT_AVAILABLE, &mut available);
        }
        if available == gl::TRUE as _ {
            let mut result = 0;
            unsafe {
                gl::GetQueryObjectiv(self.0.get(), gl::QUERY_RESULT, &mut result);
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn get_result_u32(&self) -> Option<u32> {
        let mut available = 0;
        unsafe {
            gl::GetQueryObjectuiv(self.0.get(), gl::QUERY_RESULT_AVAILABLE, &mut available);
        }
        if available == gl::TRUE as _ {
            let mut result = 0;
            unsafe {
                gl::GetQueryObjectuiv(self.0.get(), gl::QUERY_RESULT, &mut result);
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn is_allocated(&self) -> bool {
        unsafe {
            gl::IsQuery(self.0.get()) == gl::TRUE
        }
    }
}

pub fn get_counter_bits(target: QueryTarget) -> i32 {
    let mut result = 0;
    unsafe {
        gl::GetQueryiv(target as u32, gl::QUERY_COUNTER_BITS, &mut result);
    }
    result
}

pub fn get_current_query(target: QueryTarget) -> Option<NonZeroU32> {
    let mut result = 0;
    unsafe {
        gl::GetQueryiv(target as u32, gl::CURRENT_QUERY, &mut result);
    }
    NonZeroU32::new(result as u32)
}