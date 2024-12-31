use crate::{
    effect::{ 
        particle::ParticleEmitterType,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{
        curve::{ Curve2, Curve4 },
        resources::ResBuffer
    },
    utility::misc::RGBA
};
use glam::{ Vec3, Vec3A, Mat4 };

#[repr(C)]
#[derive(Debug)]
pub struct PolygonTrack {
    transform: Mat4,
    scale: Vec3A,
    type_: ParticleEmitterType,
    number: u32,
    color: RGBA,
    time: f32,
    refresh: f32,
    seed: u32,
    dirty: u32,
    material: *mut EPLMaterial,
    vertex_buffer: *mut u8,
    command: [*mut ResBuffer; 2],
    job_data: *mut u8,
    parts: *mut u8,
    params: *mut EPLParameter
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParts {
    points: [Vec3; 32],
    time: f32,
    step: f32,
    current: u8,
    count: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    number: u32,
    thick_core: f32,
    thick_outside: f32,
    rgb_core: Curve4,
    rgb_outside: Curve4,
    alpha: Curve2,
    split: u32,
    repeat_tex_v: f32,
    move_tex_v: f32,
    interval: f32,
    field13c: u32
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
