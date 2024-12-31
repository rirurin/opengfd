use crate::{
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::resources::ResBuffer,
    utility::misc::{ Fade, Range, RGBA }
};
use glam::{ Vec3A, Mat4 };

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThunderPolygonType
{
    Bar = 1,
    Sphere
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonThunder {
    transform: Mat4,
    scale: Vec3A,
    type_: ThunderPolygonType,
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
    fade: Fade,
    blend: u32,
    split: u32,
    wave: Range,
    wave_height: Range,
    uneven: Range,
    both_ends: Fade,
    width_core: f32,
    width_edge: f32,
    rgba_core: RGBA,
    rgba_border: RGBA, 
    rgba_edge: RGBA,
    adjust: Adjustment,
    seed: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BarParams {
    basic: BasicParams,
    length: Range 
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct BarParts {
    basic: BasicParts,
    wave: f32,
    wave_height: f32,
    uneven: f32,
    length: f32,
    seed: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SphereParams {
    basic: BasicParams,
    radius: Range,
    round: Range,
    rotate_y_speed: Range,
    rotate_y_accele: f32,
    rotate_y_type: u32,
    rotate_z_speed: Range,
    rotate_z_accele: f32,
    rotate_z_type: u32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SphereParts {
    basic: BasicParts,
    axis: f32,
    round: f32,
    rotate_y: f32,
    rotate_y_speed: f32,
    rotate_y_accele: f32,
    rotate_z: f32,
    rotate_z_speed: f32,
    rotate_z_accele: f32,
    radius_begin: f32,
    radius_end: f32,
    radian_y: f32,
    radian_z: f32,
    wave: f32,
    wave_height: f32,
    uneven: f32,
    seed: u32,
    refresh: f32,
}
