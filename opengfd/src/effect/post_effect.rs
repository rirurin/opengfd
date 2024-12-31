use crate::{
    effect::{
        // misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{ 
        curve::Curve4,
        // resources::ResBuffer
    },
    utility::misc::{ Fade, RGBA }
};
use glam::{ Vec2, Mat4 };

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostEffectType {
    RadialBlur = 1,
    StraightBlur,
    NoiseBlur,
    DistortionBlur,
    FillData,
    LensFlareData,
    ColorCorrectionData,
    MonotoneData,
    #[cfg(feature = "v1-core")]
    LensFlareMake,
    #[cfg(feature = "v2-core")]
    ChromaticAberration,
    #[cfg(feature = "v1-core")]
    MotionBlur,
    #[cfg(feature = "v2-core")]
    ColorCorrectionExcludeToon,
    #[cfg(feature = "v1-core")]
    AfterimageBlur
}

#[repr(C)]
#[derive(Debug)]
pub struct PostEffect {
    transform: Mat4,
    type_: PostEffectType,
    time: f32,
    color: RGBA,
    material: *mut EPLMaterial,
    params: *mut EPLParameter,
    parts: *mut u8,
    dirty: u32
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct BasicParams {
    flags: u32,
    life: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RadialBlurParams {
    basic: BasicParams,
    rgba: Curve4, // color_over_time
    blend: u32,
    power: Curve4, // intensity_over_time
    falloff: f32,
    center_x: f32, // blur_focal_point
    center_y: f32,
    ssao_mask: bool,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct StraightBlurParams {
    basic: BasicParams,
    rgba: Curve4,
    blend: u32,
    power: Curve4,
    direction: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct NoiseBlurParams {
    basic: BasicParams,
    rgba: Curve4,
    blend: u32,
    power: Curve4,
    scale: Curve4,
    ssao_mask: bool,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct DistortBlurParams {
    basic: BasicParams,
    rgba: Curve4,
    blend: u32,
    power: [Curve4; 2],
    uv: [Vec2; 2]
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct FillParams {
    basic: BasicParams,
    rgba: [Curve4; 4],
    blend: u32,
    power: Curve4,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct LensFlareParams {
    basic: BasicParams,
    flags: u32,
    templ: u32,
    filter: u32,
    rgba: Curve4,
    brightness: f32,
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct MonotoneParams {
    basic: BasicParams,
    alpha: f32,
    fade: Fade
}
