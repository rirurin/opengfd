use crate::{
    anim::animation::Animation,
    object::object::Object,
    utility::reference::Reference
};
use std::{
    marker::PhantomPinned,
    ptr::NonNull
};
use riri_mod_tools_proc::ensure_layout;

#[ensure_layout(size = 56usize)]
pub struct Timeline {
    #[field_offset(0usize)]
    pub field0_0x0: i32,
    #[field_offset(4usize)]
    pub field1_0x4: f32,
    #[field_offset(8usize)]
    pub field2_0x8: f32,
    #[field_offset(16usize)]
    pub anim: *mut Animation,
    #[field_offset(24usize)]
    pub field5_0x18: u32,
    #[field_offset(32usize)]
    pub head: *mut TimelineTrack,
    #[field_offset(40usize)]
    pub tail: *mut TimelineTrack,
    #[field_offset(48usize)]
    pub ref_: Reference,
}

#[repr(C)]
pub struct TimelineTrack {
    start: f32,
    end: f32,
    joint: i32,
    keys: usize,
    object: *mut Object,
    prev: Option<NonNull<TimelineTrack>>,
    next: Option<NonNull<TimelineTrack>>,
    _pinned: PhantomPinned
}
