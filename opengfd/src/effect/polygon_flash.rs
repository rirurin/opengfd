use crate::{
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{
        curve::{ Curve2, Curve4 },
        resources::ResBuffer
    },
    utility::misc::{ Fade, Range, RGB, RGBA }
};
use glam::{ Vec3A, Mat4 };

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlashPolygonType {
    Radiation = 1,
    Explosion,
    Ring,
    Scatter,
    Cylinder
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonFlash {
    transform: Mat4,
    scale: Vec3A,
    type_: FlashPolygonType,
    number: u32,
    color: RGBA,
    time: f32,
    refresh: f32,
    dirty: u32,
    seed: u32,
    base: u32,
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
    flag: u32,
    life: f32,
    number: u32,
    count: Range,
    alpha: f32,
    fade: Fade,
    random: f32,
    blend: u32,
    scale: Curve2,
    adjust: Adjustment,
    seed: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct RadiateParams {
    basic: BasicParams,
    rgba_in: RGBA,
    rgba_out: RGBA,
    width_in: Range,
    width_out: Range,
    length: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RadiateParts {
    basic: BasicParts,
    width_in: f32,
    width_out: f32,
    length: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Range,
    rgba_in: RGBA,
    rgba_out: RGBA,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SparkParts {
    basic: BasicParts,
    width: f32,
    length: f32,
    speed: f32,
    radian: f32,
    position: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct RingParams {
    basic: BasicParams,
    radius: Range,
    spread: Range,
    rgba_in: RGBA,
    rgba_out: RGBA,
    width_in: Range,
    width_out: Range,
    length: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RingParts {
    basic: BasicParts,
    width_in: f32,
    width_out: f32,
    length: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    spread_start: f32,
    spread_end: f32,
    radius: f32,
    spread: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ScatterParams {
    basic: BasicParams,
    radius: Curve2,
    spread: Curve2,
    rgba_in: RGBA,
    rgba_out: RGBA,
    width_in: Range,
    width_out: Range,
    length: Range,
    speed: Range,
    accele: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct ScatterParts {
    basic: BasicParts,
    width_in: f32,
    width_out: f32,
    length: f32,
    speed: f32,
    radian: f32,
    radius: f32,
    spread_start: f32,
    spread_end: f32,
    spread: f32,
    move_: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct CylinderParams {
    basic: BasicParams,
    radius: Curve2,
    rgb: Curve4,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct CylinderParts {
    basic: BasicParts,
    width: f32,
    length: f32,
    speed: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radius: f32,
    radius_start: f32,
    radius_end: f32,
    radian: f32,
    move_: f32,
    rgb: RGB,
}
