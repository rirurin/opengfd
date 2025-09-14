use bitflags::bitflags;
use crate::{
    kernel::{
        allocator::GfdAllocator,
        task::{
            Task as GfdTask,
            TaskList
        },
    },
    utility::mutex::{ Mutex, RecursiveMutex }
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct GlobalFlags: u32 {
        const NO_INCREASE_TASK_START_TIMER = 1 << 0;
        const RENDER_TASK_RUNNING = 1 << 1;
        /// True if _pre_<GFD> or _post_<GFD> are executing
        const SYSTEM_TASK_RUNNING = 1 << 2;
        const RESIZE_FLAG0 = 1 << 3;
        const RESIZE_FLAG1 = 1 << 4;
        const Flag5 = 1 << 5;
        const Flag6 = 1 << 6;
        const Flag7 = 1 << 7;
        const Flag8 = 1 << 8;
        const Flag9 = 1 << 9;
        const Flag10 = 1 << 10;
        const Flag11 = 1 << 11;
        const Flag12 = 1 << 12;
        const Flag13 = 1 << 13;
        const Flag14 = 1 << 14;
        const Flag15 = 1 << 15;
        const Flag16 = 1 << 16;
        const Flag17 = 1 << 17;
        const Flag18 = 1 << 18;
        const Flag19 = 1 << 19;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
        const Flag25 = 1 << 25;
        const Flag26 = 1 << 26;
        const Flag27 = 1 << 27;
        const Flag28 = 1 << 28;
        const Flag29 = 1 << 29;
        const Flag30 = 1 << 30;
        const Flag31 = 1 << 31;
    }
}

// TASK START

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TaskFlag : u32 {
        const PRE_GFD = 1 << 2;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct TaskGlobal {
    pub flag: TaskFlag,
    pub begin: TaskList<GfdAllocator>,
    pub update: TaskList<GfdAllocator>,
    pub render: TaskList<GfdAllocator>,
    pub end: TaskList<GfdAllocator>,
    pub release: TaskList<GfdAllocator>,
    pub task_count: u32,
    pub pad: TaskList<GfdAllocator>,
    pub detach_mutex: *mut GfdTask<GfdAllocator>,
    pub current: *mut GfdTask<GfdAllocator>,
    #[cfg(feature = "v2-core")]
    pub mutex: Mutex,
    #[cfg(feature = "v1-core")]
    pub mutex: RecursiveMutex
}

impl TaskGlobal {
    pub fn get_first_begin_task(&self) -> *const GfdTask<GfdAllocator> {
        self.begin.get_head_ptr()
    }
    pub fn get_first_begin_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.begin.get_head_ptr()
    }
    pub fn get_first_update_task(&self) -> *const GfdTask<GfdAllocator> {
        self.update.get_head_ptr()
    }
    pub fn get_first_update_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.update.get_head_ptr()
    }
    pub fn get_first_render_task(&self) -> *const GfdTask<GfdAllocator> {
        self.render.get_head_ptr()
    }
    pub fn get_first_render_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.render.get_head_ptr()
    }
    pub fn get_first_ending_task(&self) -> *const GfdTask<GfdAllocator> {
        self.end.get_head_ptr()
    }
    pub fn get_first_ending_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.end.get_head_ptr()
    }

    pub fn get_last_begin_task(&self) -> *const GfdTask<GfdAllocator> {
        self.begin.get_tail_ptr()
    }
    pub fn get_last_begin_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.begin.get_tail_ptr()
    }
    pub fn get_last_update_task(&self) -> *const GfdTask<GfdAllocator> {
        self.update.get_tail_ptr()
    }
    pub fn get_last_update_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.update.get_tail_ptr()
    }
    pub fn get_last_render_task(&self) -> *const GfdTask<GfdAllocator> {
        self.render.get_tail_ptr()
    }
    pub fn get_last_render_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.render.get_tail_ptr()
    }
    pub fn get_last_ending_task(&self) -> *const GfdTask<GfdAllocator> {
        self.end.get_tail_ptr()
    }
    pub fn get_last_ending_task_mut(&self) -> *mut GfdTask<GfdAllocator> {
        self.end.get_tail_ptr()
    }

    pub fn set_first_begin_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.begin.set_head(val);
    }
    pub fn set_first_update_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.update.set_head(val);
    }
    pub fn set_first_render_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.render.set_head(val);
    }
    pub fn set_first_ending_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.end.set_head(val);
    }
    pub fn set_last_begin_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.begin.set_tail(val);
    }
    pub fn set_last_update_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.update.set_tail(val);
    }
    pub fn set_last_render_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.render.set_tail(val);
    }
    pub fn set_last_ending_task(&mut self, val: *mut GfdTask<GfdAllocator>) {
        self.end.set_tail(val);
    }
}

// TASK END
