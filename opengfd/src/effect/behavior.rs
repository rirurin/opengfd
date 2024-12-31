use crate::{
    effect::{
        misc::Adjustment,
        particle::ParticleEmitterType
    },
    graphics::curve::Curve4,
    utility::misc::{ Fade, Range }
};
use glam::{ Vec2, Vec3, Mat4 };

#[repr(C)]
#[derive(Debug)]
pub struct Behavior {
    type_: ParticleEmitterType,
    number: u32,
    num_all: u32,
    generated: u32,
    time: f32,
    refresh: f32,
    scale: f32,
    seed: u32,
    base: u32,
    dirty: u32,
    posture: *mut EPLPosture,
    physics: *mut u8,
    params: *mut u8,
    initial: *mut u8,
    job_data: *mut u8
}

#[repr(C)]
#[derive(Debug)]
pub struct EPLPosture {
    pos: Vec3,
    time: f32
}

// <gfdsharp> ParticleEmitter

#[repr(C)]
#[derive(Debug)]
pub struct EmitterParams {
    transform: Mat4,
    life: f32, // despawn_timer
    count: Range // spawn_choker
}

#[repr(C)]
#[derive(Debug)]
pub struct ColorParams {
    curve4: Curve4, // color_over_life
    alpha: f32,
    fade: Fade,
    random: f32,
    blend: u32 // draw_queue_id
}

#[repr(C)]
#[derive(Debug)]
pub struct ScaleParams {
    curve4: Curve4, // size_over_life
    rate: Vec2 // field12c/field130
}

#[repr(C)]
#[derive(Debug)]
pub struct RotateParams {
    start: Range, // spawner_angles
    speed: Range,
    accele: f32,
    rotate_type: u32 // field148
}

#[repr(C)]
#[derive(Debug)]
pub struct BlurParams {
    length: u32, // field14c
    interval: f32 // field150
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    emitter: EmitterParams,
    color: ColorParams,
    scale: ScaleParams,
    rotate: RotateParams,
    blur: BlurParams,
    adjust: Adjustment,
    flags: u32, // random_spawn_delay
    life: f32, // particle_life
    seed: u32 // angle_seed
}

#[repr(C)]
#[derive(Debug)]
pub struct SmokeParams {
    basic: BasicParams,
    radius: f32, // disperse_dist
    speed: Range, // launch_up_speed, launch_up_speed_rand
    gravity: Range, // gravity_strength, gravity_strength_rand
    amplif_begin: Range, // disperse_start_dist, disperse_start_dist_rand
    amplif_end: Range, // disperse_end_dist, disperse_end_dist_rand
    amplif_speed: Range // disperse_rotate, disperse_rotate_rand
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Range,
    spread_xz: Range,
    spread_y: Range,
    speed: Range,
    gravity: Range,
    accele: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SpiralParams {
    basic: BasicParams,
    height: f32, // spiral_height
    radius_begin: Range, // spiral_start_radius
    radius_end: Range, // spiral_end_radius
    rotate_speed: Range, // soft_spiral_rotation
    rotate_accele: f32, // hard_spiral_rotation
    rise_speed: Range, // slide_up_speed
    gravity: Range // slide_down_speed
}

#[repr(C)]
#[derive(Debug)]
pub struct SphereParams {
    basic: BasicParams,
    radius_begin: Range,
    radius_end: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    gravity: Range,
}

#[repr(C)]
#[derive(Debug)]
pub struct RingParams {
    basic: BasicParams,
    radius: Range,
    height: Range,
    spread_speed: Range,
    spread_accele: f32,
    rotate_speed: Range,
    rotate_accele: f32,
    gravity: Range,
}

#[repr(C)]
#[derive(Debug)]
pub struct LineParams {
    basic: BasicParams,
    length: f32,
    speed: Range,
    gravity: Range,
    ampli_begin: Range,
    ampli_end: Range,
    ampli_speed: Range,
}
