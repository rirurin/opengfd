use crate::{
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{
        curve::Curve4,
        resources::ResBuffer
    },
    utility::misc::{ Fade, Range, RGBA }
};
use glam::{ Vec2, Vec3A, Mat4 };

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlashPolygonType {
    Square = 1,
    Oblong
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonBoard {
    transform: Mat4,
    scale: Vec3A,
    type_: FlashPolygonType,
    color: RGBA,
    time: f32,
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
    flags: u32, // render_type
    life: f32, // anim_length
    fade: Fade,
    blend: u32, // layer_mode
    adjust: Adjustment,
    seed: u32
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SquareParts {
    basic: BasicParts,
    rotate_start: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SquareParams {
    basic: BasicParams,
    pivot: Vec2,
    length: Curve4,
    rgb: Curve4,
    alpha: f32,
    rotate_start: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct OblongParts {
    basic: BasicParts,
    rotate_start: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct OblongParams {
    basic: BasicParams,
    pivot: Vec2,
    width: Curve4,
    height: Curve4,
    rgb: Curve4,
    alpha: f32,
    rotate_start: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
}
