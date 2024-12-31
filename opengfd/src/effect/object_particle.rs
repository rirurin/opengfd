use crate::{
    effect::{ 
        behavior::Behavior,
        particle::ParticleEmitterType,
        resources::{ EPLParameter, EPLMaterial }
    },
    object::node::Node,
    utility::misc::RGBA
};
use glam::{ Vec3A, Quat };

#[repr(C)]
#[derive(Debug)]
pub struct ObjectParticle {
    type_: ParticleEmitterType,
    number: u32,
    scale: f32,
    color: RGBA,
    behavior: *mut Behavior,
    material: *mut EPLMaterial,
    parts: *mut u8,
    params: *mut EPLParameter,
    dirty: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParts {
    rotate0: Quat,
    axis: Vec3A,
    node: *mut Node,
    handle: *mut u8
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    number: u32,
    axis: [f32; 3]
}

#[repr(C)]
#[derive(Debug)]
pub struct SmokeParams {
    basic: BasicParams,
    behavior: super::behavior::SmokeParams
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    behavior: super::behavior::SparkParams
}

#[repr(C)]
#[derive(Debug)]
pub struct SpiralParams {
    basic: BasicParams,
    behavior: super::behavior::SpiralParams
}

#[repr(C)]
#[derive(Debug)]
pub struct SphereParams {
    basic: BasicParams,
    behavior: super::behavior::SphereParams
}

#[repr(C)]
#[derive(Debug)]
pub struct RingParams {
    basic: BasicParams,
    behavior: super::behavior::RingParams
}

#[repr(C)]
#[derive(Debug)]
pub struct LineParams {
    basic: BasicParams,
    behavior: super::behavior::LineParams
}
