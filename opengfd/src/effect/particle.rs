use crate::{
    effect::{
        behavior::Behavior,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::resources::ResBuffer,
    utility::misc::RGBA
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParticleEmitterType {
    Smoke = 1,
    Explosion,
    Spiral,
    Ball,
    Circle,
    StraightLine
}

#[repr(C)]
#[derive(Debug)]
pub struct Particle {
    emitter_type: ParticleEmitterType,
    number: u32,
    color: RGBA,
    behavior: *mut Behavior,
    material: *mut EPLMaterial,
    vertex_buffer: *mut u8,
    command: [*mut ResBuffer; 2],
    job_data: *mut u8,
    params: *mut EPLParameter 
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32
}
