#![allow(dead_code)]
use glam::Vec3;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoundingBox {
    max: Vec3,
    min: Vec3
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoundingSphere {
    center: Vec3,
    radius: f32
}
#[derive(Debug, Clone, Copy)]
pub struct RGB(glam::U8Vec3);
#[derive(Debug, Clone, Copy)]
pub struct RGBA(glam::U8Vec4);
#[derive(Debug, Clone, Copy)]
pub struct RGBFloat(glam::Vec3);
#[derive(Debug, Clone, Copy)]
pub struct RGBAFloat(glam::Vec4);

#[repr(C)]
#[derive(Debug)]
pub struct Fade {
    in_: f32,
    out_: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct Range {
    datums: f32,
    range: f32
}
