use crate::{
    object::node::Node,
    utility::reference::Reference
};
use glam::{ Vec2, Vec4 };

#[repr(C)]
pub struct BipedIK {
    parts_mask: u32,
    num_recursive: u32,
    parts: [BipedIKParts; 8],
    ref_: Reference
}

#[repr(C)]
pub struct BipedIKParts {
    target: Vec4,
    flags: u8,
    node: *mut Node,
    node_count: u32,
    nodes: [BipedIKNode; 5],
    ground: *mut Node
}

#[repr(C)]
pub struct BipedIKNode {
    p: *mut Node,
    limit: [Vec2; 3] // min, max
}
