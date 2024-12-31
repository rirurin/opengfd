use crate::{
    anim::key::KeyController,
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    object::node::Node,
    utility::misc::{ Fade, RGBA }
};
use glam::{ Vec3A, Mat4 };
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LightType
{
    Mesh = 1,
    Scene
}

#[repr(C)]
#[derive(Debug)]
pub struct Camera {
    transform: Mat4,
    scale: Vec3A,
    type_: LightType,
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
pub struct MeshParams {
    basic: BasicParams,
}

#[repr(C)]
#[derive(Debug)]
pub struct SceneParams {
    basic: BasicParams,
    life: f32,
    fade: Fade,
    ambient: u32,
    diffuse: u32,
    specular: u32,
    rotate: [f32; 3],
}
