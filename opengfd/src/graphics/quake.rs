use glam::Vec3A;
use crate::utility::reference::Reference;

#[repr(C)]
#[derive(Debug)]
pub struct QuakeParams {
    power: f32,
    pitch_weight: f32,
    total_time: f32,
    fade_in_time: f32,
    fade_out_time: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Quake {
    translate: Vec3A,
    time: f32,
    params: QuakeParams,
    flags: u16,
    state: u16,
    ref_: Reference
}
