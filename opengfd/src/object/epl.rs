use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    anim::timeline::Timeline,
    effect::parts::Part,
    kernel::allocator::GfdAllocator,
    utility::{
        item_array::ItemArray,
        misc::{ RGBA, Fade, Range },
        name::Name,
        reference::Reference
    }
};
use glam::Vec3A;
use super::{
    node::Node,
    object::Object
};
use std::ptr::NonNull;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplFlags : u32 {
        const Flag0 = 1 << 0;
        const Flag1 = 1 << 1;
        const Flag2 = 1 << 2; // Set on load
        const Flag3 = 1 << 3;
        const Flag4 = 1 << 4;
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

#[repr(C)]
pub struct EPL<A = GfdAllocator> 
where A: Allocator + Clone
{
    super_: Object<A>,
    scale: Vec3A,
    flag: EplFlags,
    dirty: i32,
    time: f32,
    root: *mut Node<A>,
    timeline: *mut Timeline,
    leaves: *mut ItemArray<EPLLeaf, A>,
    rgba: RGBA,
    frequency: f32,
    field60: [u8; 0x18],
    field35_0x78: *mut u8,
    field80: u32,
    ref_: Reference,
    _allocator: A
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplLeafFlags : u32 {
        const Flag0 = 1 << 0;
        const Flag1 = 1 << 1;
        const Flag2 = 1 << 2;
        const Flag3 = 1 << 3;
        const Flag4 = 1 << 4;
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

#[repr(C)]
#[derive(Debug)]
pub struct EPLLeaf {
    _super: Object,
    scale: Vec3A,
    color: RGBA,
    fade: Fade,
    range: Range,
    rgba: RGBA,
    key_rgba: RGBA,
    field_4c: u8,
    flags: EplLeafFlags,
    name: Name,
    parts: Option<NonNull<Part>> 
}

#[repr(C)]
#[derive(Debug)]
pub struct EPLParts {

}
