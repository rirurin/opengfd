use bitflags::bitflags;
use crate::{
    anim::timeline::Timeline,
    effect::parts::Part,
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
use riri_mod_tools_proc::ensure_layout;

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

#[ensure_layout(size = 144usize)]
pub struct EPL {
    #[field_offset(0usize)]
    pub super_: Object,
    #[field_offset(32usize)]
    pub scale: Vec3A,
    #[field_offset(48usize)]
    pub flag: EplFlags,
    #[field_offset(52usize)]
    pub dirty: i32,
    #[field_offset(56usize)]
    pub time: f32,
    #[field_offset(64usize)]
    pub root: *mut Node,
    #[field_offset(72usize)]
    pub timeline: *mut Timeline,
    #[field_offset(80usize)]
    pub leaves: *mut ItemArray<EPLLeaf>,
    #[field_offset(88usize)]
    pub rgba: RGBA,
    #[field_offset(92usize)]
    pub frequency: f32,
    #[field_offset(120usize)]
    pub field35_0x78: *mut ::std::os::raw::c_void,
    #[field_offset(132usize)]
    pub ref_: Reference,
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
