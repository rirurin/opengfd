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
use glam::{ Vec3, Vec3A, Mat4 };

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindPolygonType
{
    Spiral = 1,
    Spark,
    Sphere
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonWind {
    transform: Mat4,
    scale: Vec3A,
    type_: WindPolygonType,
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
#[allow(dead_code)]
#[derive(Debug)]
pub struct BasicParts(f32);

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    life: f32, 
    number: u32, 
    count: Range, 
    rgb: Curve4, 
    alpha: f32, 
    fade: Fade, 
    random: f32, 
    blend: u32, 
    split: u32, 
    repeat_tex_v: Range, 
    move_tex_v: Range, 
    adjust: Adjustment, 
    seed: u32, 
}

#[repr(C)]
#[derive(Debug)]
pub struct SpiralParams {
    basic: BasicParams,
    radius: Curve2,
    height: Curve2,
    width: Curve2,
    slant: Range,
    round: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
    both_ends: Fade,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SprialParts {
    basic: BasicParts,
    round: f32,
    slant: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radius_begin: f32,
    radius_end: f32,
    height_begin: f32,
    height_end: f32,
    width_begin: f32,
    width_end: f32,
    repeatv: f32,
    move_v: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Curve2,
    width_ratio: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
    move_tex_v: Range,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SparkParts {
    basic: BasicParts,
    axis: Vec3,
    radius_begin: f32,
    radius_end: f32,
    width_ratio: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    repeat_u: f32,
    move_u: f32,
    move_v: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SphereParams {
    basic: BasicParams,
    radius: Curve2,
    width: Curve2,
    round: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
    both_ends: Fade,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SphereParts {
    basic: BasicParts,
    axis: Vec3,
    round: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radius_begin: f32,
    radius_end: f32,
    width_begin: f32,
    width_end: f32,
    repeat_v: f32,
    move_v: f32,
    radian: f32,
}
