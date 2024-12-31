#![allow(dead_code)]
//! NOTE: Float arrays are used instead of glam's vectors to keep alignemnt value as alignof(f32)

use glam::Vec3;
use std::ptr::NonNull;

pub trait AnimKey {

}
// BASIC TYPES

#[derive(Debug)]
pub struct KeyFloat(f32);
impl AnimKey for KeyFloat {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyTRS {
    t: [f32; 3],
    r: [f32; 4],
    s: [f32; 3],
}
impl AnimKey for KeyTRS {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyRGB([f32; 3]);
impl AnimKey for KeyRGB {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyRGBA([f32; 4]);
impl AnimKey for KeyRGBA {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyUV {
    offset: [f32; 2],
    tile: [f32; 2],
    angle: f32
}
impl AnimKey for KeyUV {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyController {
    flags: u32,
    anim: u32,
    blend: f32,
    weight: f32,
    speed: f32,
}
impl AnimKey for KeyController {}

// OBJECT TYPES

#[repr(C)]
#[derive(Debug)]
pub struct KeyNode {
    dirty: u32,
    trs: KeyTRS,
}
impl AnimKey for KeyNode {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyMaterial {
    dirty: u32,
    ambient: KeyRGB,
    diffuse: KeyRGB,
    specular: KeyRGB,
    shininess: KeyFloat,
    reflect: KeyRGBA,
    diffusivity: KeyFloat,
    transparency: KeyFloat,
    uv: KeyUV,
    emissive: KeyRGB,
    reflection: KeyFloat,
    uv_multiple: KeyUV,
    transparency_multiple: KeyFloat
}
impl AnimKey for KeyMaterial {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyShape {
    dirty: u32,
    weight: KeyFloat,
    padding: [u8; 8]
}
impl AnimKey for KeyShape {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyCamera {
    dirty: u32,
    fovy: KeyFloat,
    roll: KeyFloat
}
impl AnimKey for KeyCamera {}

#[repr(C)]
#[derive(Debug)]
pub struct KeyLight {
    dirty: u32,
    diffuse: KeyRGB
}
impl AnimKey for KeyLight {}

#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum KeyType {
    Invalid = 0,
    NodePR = 1,
    NodePRS = 2,
    Vector3 = 3,
    Quaternion = 4,
    Single = 5,
    Vector3_2 = 6,
    Vector3_3 = 7,
    Vector3_4 = 8,
    Single_2 = 9,
    Quaternion_2 = 10,
    Single_3 = 11,
    MaterialSingle_4 = 12,
    Single5 = 13,
    MaterialVector3_5 = 14,
    Single_5 = 15,
    Single_6 = 16,
    PRSByte = 17,
    Single3Byte = 18,
    SingleByte = 19,
    Single5_2 = 20,
    Single5Alt = 21,
    Type22 = 22,
    CameraFieldOfView = 23,
    Single_8 = 24,
    SingleAlt_2 = 25,
    NodePRHalf = 26,
    NodePRSHalf = 27,
    NodePRHalf_2 = 28,
    MaterialSingle_9 = 29,
    SingleAlt_3 = 30,
    Type31 = 31,
    NodeRHalf = 32,
    NodeSHalf = 33,
	P5R_34 = 34,
	P5R_35 = 35,
    P5R_36 = 36,
}

#[repr(C)]
pub struct KeyMaxDiff {
    t: Vec3,
    s: Vec3
}

#[repr(C)]
pub struct KeyList {
    key_type: KeyType,
    key_count: u32,
    keys: Option<NonNull<u8>>,
    times: Option<NonNull<f32>>,
    diff: KeyMaxDiff
}
