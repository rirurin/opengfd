use crate::{
    anim::key::KeyController,
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    object::node::Node,
    utility::misc::RGBA
};
use glam::{ Vec3A, Mat4 };
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshType
{
    TwoD = 1,
    ThreeD 
}

#[repr(C)]
#[derive(Debug)]
pub struct Camera {
    transform: Mat4,
    scale: Vec3A,
    type_: MeshType,
    color: RGBA, 
    dirty: u32,
    current: KeyController,
    next: KeyController,
    material: *mut EPLMaterial,
    node: *mut Node,
    parts: *mut u8,
    params: *mut EPLParameter
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct BasicParts(f32);

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    adjust: Adjustment,
}

#[repr(C)]
#[derive(Debug)]
pub struct TwoDParams {
    basic: BasicParams,
    distance: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct ThreeDParams {
    basic: BasicParams,
}
