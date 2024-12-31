use crate::{
    anim::key::KeyController,
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    object::node::Node,
};
use glam::{ Vec3A, Mat4 };
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CameraType
{
    Mesh = 1,
    Quake
}

#[repr(C)]
#[derive(Debug)]
pub struct Camera {
    transform: Mat4,
    scale: Vec3A,
    type_: CameraType,
    dirty: u32,
    seed: u32,
    current: KeyController,
    next: KeyController,
    material: *mut EPLMaterial,
    index: u32,
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
    seed: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshParams {
    basic: BasicParams,
}

#[repr(C)]
#[derive(Debug)]
pub struct QuakeParams {
    basic: BasicParams,
    quake: crate::graphics::quake::QuakeParams
}
