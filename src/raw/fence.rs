use crate::gl;
use crate::gl::{GLenum, GLsync};
use crate::state::GraphicsContext;

#[repr(transparent)]
pub struct Sync(GLsync, pub(crate) GraphicsContext);

impl GraphicsContext {
    pub fn fence(&self, condition: SyncCondition) -> Sync {
        let sync = unsafe {
            gl::FenceSync(condition as _, 0)
        };
        assert!(!sync.is_null(), "fence is null");
        Sync(sync, *self)
    }
}

impl Sync {
    pub fn ty(&self) -> SyncType {
        let mut ty = 0;
        let mut len = 0;
        unsafe {
            gl::GetSynciv(self.0, gl::OBJECT_TYPE, 1, &mut len, &mut ty);
        }
        assert_eq!(len, 1);
        match ty as GLenum {
            gl::SYNC_FENCE => SyncType::Fence,
            _ => unreachable!(),
        }
    }

    pub fn condition(&self) -> SyncCondition {
        let mut ty = 0;
        let mut len = 0;
        unsafe {
            gl::GetSynciv(self.0, gl::SYNC_CONDITION, 1, &mut len, &mut ty);
        }
        assert_eq!(len, 1);
        match ty as GLenum {
            gl::SYNC_GPU_COMMANDS_COMPLETE => SyncCondition::GpuCommandsComplete,
            _ => unreachable!(),
        }
    }

    pub fn status(&self) -> SyncStatus {
        let mut ty = 0;
        let mut len = 0;
        unsafe {
            gl::GetSynciv(self.0, gl::SYNC_STATUS, 1, &mut len, &mut ty);
        }
        assert_eq!(len, 1);
        match ty as GLenum {
            gl::SIGNALED => SyncStatus::Signaled,
            gl::UNSIGNALED => SyncStatus::Unsignaled,
            _ => unreachable!(),
        }
    }

    pub fn wait(&self) {
        unsafe {
            gl::WaitSync(self.0, 0, gl::TIMEOUT_IGNORED);
        }
    }

    pub fn poll(&self, timeout_ns: u64) -> PollResult {
        match unsafe {
            gl::ClientWaitSync(self.0, gl::SYNC_FLUSH_COMMANDS_BIT, timeout_ns)
        } {
            gl::ALREADY_SIGNALED => PollResult::AlreadySignaled,
            gl::TIMEOUT_EXPIRED => PollResult::TimeoutExpired,
            gl::CONDITION_SATISFIED => PollResult::ConditionSatisfied,
            gl::WAIT_FAILED => PollResult::WaitFailed,
            _ => unreachable!(),
        }
    }
}

impl Drop for Sync {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteSync(self.0);
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum PollResult {
    AlreadySignaled = gl::ALREADY_SIGNALED,
    TimeoutExpired = gl::TIMEOUT_EXPIRED,
    ConditionSatisfied = gl::CONDITION_SATISFIED,
    WaitFailed = gl::WAIT_FAILED,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum SyncType {
    Fence = gl::SYNC_FENCE,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum SyncStatus {
    Signaled = gl::SIGNALED,
    Unsignaled = gl::UNSIGNALED,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
#[non_exhaustive]
pub enum SyncCondition {
    GpuCommandsComplete = gl::SYNC_GPU_COMMANDS_COMPLETE,
}

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum SyncFlags {
}