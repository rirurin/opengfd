use crate::utility::name::Name;
use std::marker::PhantomPinned;

#[repr(C)]
#[derive(Debug)]
pub struct TaskLink {
    pub parent: *mut Task,
    pub child: *mut Task,
    pub sibling: *mut Task,
}

#[repr(C)]
#[derive(Debug)]
pub struct TaskListNode {
    prev: *mut Task,
    next: *mut Task
}

#[repr(C)]
#[derive(Debug)]
pub struct TaskList {
    head: *mut Task,
    tail: *mut Task
}

#[repr(C)]
#[derive(Debug)]
pub struct Task {
    pub status: i32,
    pub flags: i32,
    pub update_prio: i32,
    pub render_prio: i32,
    pub name: Name,
    pub start_timer: i32,
    pub shutdown_timer: i32,
    pub update_fn: Option<fn(*mut u8, f32) -> u64>,
    pub render_fn: Option<fn(*mut u8) -> u64>,
    pub end_fn: Option<fn(*mut u8) -> ()>,
    pub main_work: *mut u8,
    // NOTE: This field isn't in P5R PS4
    #[cfg(feature = "adapter-hedge")]
    mouse_work: *mut u8,
    link: TaskLink,
    pub idle: TaskListNode,
    pub update: TaskListNode,
    pub render: TaskListNode,
    pub pad: TaskListNode,
    pub detach: *mut Task,
    pub uid: u64,
    _pinned: PhantomPinned
}
