use crate::{
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{
        curve::{ Curve2, Curve4 },
        resources::ResBuffer
    },
    utility::misc::{ Fade, Range, RGBA }
};
use glam::{ Vec3A, Mat4 };

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GlitterPolygonType
{
    Explosion = 1,
    Splash,
    Cylinder,
    Wall
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonGlitter {
    transform: Mat4,
    scale: Vec3A,
    type_: GlitterPolygonType,
    number: u32,
    color: RGBA,
    time: f32,
    refresh: f32,
    seed: u32,
    base: u32,
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
    time: f32,
    rgba_in: RGBA,
    rgba_out: RGBA
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    life: f32,
    number: u32,
    count: Range,
    fade: Fade,
    random: f32,
    blend: u32,
    rgba_in: Curve4,
    rgba_out: Curve4,
    scale: Curve2,
    repeat_tex_v: f32,
    move_tex_v: f32,
    adjust: Adjustment,
    seed: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Range,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SparkParts {
    basic: BasicParts,
    radius: f32,
    width: f32,
    length: f32,
    speed: f32,
    radian: f32,
    position: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ScatterParams {
    basic: BasicParams,
    radius: Curve2,
    spread: Curve2,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct ScatterParts {
    basic: BasicParts,
    width: f32,
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
    radius: Range,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
    rotate_speed: Range,
    rotaate_accele: f32,
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
}

#[repr(C)]
#[derive(Debug)]
pub struct WallParams {
    basic: BasicParams,
    distance: Curve2,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct WallParts {
    basic: BasicParts,
    width: f32,
    length: f32,
    speed: f32,
    distance: f32,
    distance_start: f32,
    distance_end: f32,
    move_: f32,
}
