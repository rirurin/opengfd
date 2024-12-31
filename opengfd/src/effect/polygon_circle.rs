use crate::{
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{
        curve::{ Curve2, Curve4 },
        resources::ResBuffer
    },
    utility::misc::{ Fade, RGBA }
};
use glam::{ Vec3A, Mat4 };

#[repr(u32)]
#[derive(Debug)]
pub enum CirclePolygonType {
    Ring = 1,
    Trajectory,
    Fill,
    Hoop
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonBoard {
    transform: Mat4,
    scale: Vec3A,
    type_: CirclePolygonType,
    color: RGBA,
    time: f32,
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
#[allow(dead_code)]
#[derive(Debug)]
pub struct BasicParts(f32);

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    life: f32,
    fade: Fade,
    blend: u32,
    adjust: Adjustment,
    seed: u32,
    split: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct RingParams {
    basic: BasicParams,
    occurrence: f32,
    radius: Curve2,
    rgba_in: Curve4,
    rgba_out: Curve4,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RingParts {
    basic: BasicParts
}

#[repr(C)]
#[derive(Debug)]
pub struct TrackParams {
    basic: BasicParams,
    radius: f32,
    width_ratio: f32,
    rgba_in: Curve4,
    rgba_out: Curve4,
    speed: f32,
    accele: f32,
    repeat_tex_u: f32,
    move_tex_v: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct TrackParts {
    basic: BasicParts,
    position: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct FillParams {
    basic: BasicParams,
    occurrence: f32,
    radius: Curve2,
    rgba_in: Curve4,
    rgba_out: Curve4,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct FillParts {
    basic: BasicParts
}

#[repr(C)]
#[derive(Debug)]
pub struct HoopParams {
    basic: BasicParams,
    occurrence: f32,
    radius: f32,
    speed: f32,
    accele: f32,
    width: Curve2,
    width_ratio: Fade,
    rgba_in: Curve4,
    rgba_center: Curve4,
    rgba_out: Curve4,
    repeat_tex_u: f32,
    move_tex_v: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct HoopParts {
    basic: BasicParts
}
