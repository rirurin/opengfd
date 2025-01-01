#![allow(dead_code)]

// SHADER INFO COMMON
pub(crate) const RENDER_STATES: usize = 33;

#[repr(C)]
#[derive(Debug)]
pub struct VideoMode {
    flags: u32,
    width: i32,
    height: i32,
    depth: i32,
    ref_rate: i32,
    format: i32
}
