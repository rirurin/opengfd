use crate::{
    anim::animation::AnimationNeck,
    object::epl::EPL,
    utility::{
        name::Name,
        reference::Reference
    }
};
use std::marker::PhantomPinned;

#[repr(C)]
pub struct AnimEffector {
    effect: *mut AnimEffectorEffect,
    neck: [AnimationNeck; 2],
    neck_current_slot: u32,
    neck_blend_time: f32,
    neck_elapsed_time: f32,
    neck_angle: [f32; 4],
    neck_animation: bool,
    ref_: Reference
}

#[repr(C)]
pub struct AnimEffectorEffect {
    epl: *mut EPL,
    name: Name,
    track: u32,
    dirty: u16,
    prev: *mut AnimEffectorEffect,
    next: *mut AnimEffectorEffect,
    _pinned: PhantomPinned
}
