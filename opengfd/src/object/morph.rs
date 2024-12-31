use crate::utility::{
    name::Name,
    reference::Reference
};
use glam::Vec3;
use super::object::Object;
use std::ptr::NonNull;
#[repr(C)]
#[derive(Debug)]
pub struct MorphController {
    _super: Object,
    num_targets: u32,
    weights: Option<NonNull<f32>>,
    active_morphs: *mut u8,
    name: Name
}

#[repr(C)]
#[derive(Debug)]
pub struct MorphTarget {
    initial: Option<NonNull<Shape>>,
    targets: Option<NonNull<Shape>>,
    num_targets: i32,
    fvf: u32,
    ref_: Reference
}

#[repr(C)]
#[derive(Debug)]
pub struct Shape {
    fvf: u32,
    num_vertices: i32,
    stride: i32,
    vertices: Option<NonNull<Vec3>>
}
