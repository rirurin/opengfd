use glam::{ Vec3A, Quat, Mat4 };
use super::object::Object;
use std::ptr::NonNull;
use crate::utility::{ name::Name, property::Property };

#[repr(C)]
#[derive(Debug)]
pub struct Node {
    _super: Object,
    world_tm: Mat4,
    local_tm: Mat4,
    transform: NodeTransform,
    link: NodeLink,
    name: Name,
    visibility: f32,
    object_head: Option<NonNull<Object>>,
    object_tail: Option<NonNull<Object>>,
    property: Option<NonNull<Property>>
}

#[repr(C)]
#[derive(Debug)]
pub struct NodeTransform {
    translate: Vec3A,
    rotate: Quat,
    scale: Vec3A
}

#[repr(C)]
#[derive(Debug)]
pub struct NodeLink {
    root: Option<NonNull<Node>>,
    child: Option<NonNull<Node>>,
    next: Option<NonNull<Node>>,
    skin_bone_index: u16,
    terminate: u16
}
