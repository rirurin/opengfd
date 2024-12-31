use crate::{
    anim::key::KeyController,
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    object::{
        epl::EPL,
        node::Node
    },
    utility::{
        item_array::ItemArray,
        misc::RGBA
    }
};
use glam::{ Vec3A, Mat4 };

#[repr(C)]
#[derive(Debug)]
pub struct Camera {
    transform: Mat4,
    scale: Vec3A,
    type_: u32,
    color: RGBA, 
    dirty: u32,
    current: KeyController,
    next: KeyController,
    material: [*mut EPLMaterial; 2],
    effect_array: ItemArray<*mut EPL>,
    node: *mut Node,
    parts: *mut u8,
    params: *mut EPLParameter
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    adjust: Adjustment,
    scale: f32
}
