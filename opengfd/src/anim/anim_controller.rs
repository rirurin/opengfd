use bitflags::bitflags;
use crate::{
    anim::{
        anim_effector::AnimEffector,
        animation::{ 
            Animation, 
            AnimInterpolator,
            AnimationNeck
        },
        biped_ik::BipedIK
    },
    utility::{
        item_array::ItemArray,
        misc::BoundingBox
    }
};

bitflags! {
    pub struct AnimationPackFlags: u32 {
        const Flag0   = 1 << 0;
        const Flag1   = 1 << 1;
        const Flag2   = 1 << 2;
        const Flag3   = 1 << 3;
        const Flag4   = 1 << 4;
        const Flag5   = 1 << 5;
        const Flag6   = 1 << 6;
        const Flag7   = 1 << 7;
        const Flag8   = 1 << 8;
        const Flag9   = 1 << 9;
        const Flag10  = 1 << 10;
        const Flag11  = 1 << 11;
        const Flag12  = 1 << 12;
        const Flag13  = 1 << 13;
        const Flag14  = 1 << 14;
        const Flag15  = 1 << 15;
        const Flag16  = 1 << 16;
        const Flag17  = 1 << 17;
        const Flag18  = 1 << 18;
        const Flag19  = 1 << 19;
        const Flag20  = 1 << 20;
        const Flag21  = 1 << 21;
        const Flag22  = 1 << 22;
        const Flag23  = 1 << 23;
        const Flag24  = 1 << 24;
        const Flag25  = 1 << 25;
        const Flag26  = 1 << 26;
        const Flag27  = 1 << 27;
        const Flag28  = 1 << 28;
        const Flag29  = 1 << 29;
        const Flag30  = 1 << 30;
        const Flag31  = 1 << 31;
    }
}


#[repr(C)]
pub struct AnimController {
    flags: AnimationPackFlags,
    interpolator: *mut AnimInterpolator,
    effector: *mut AnimEffector,
    tracks: [AnimControllerTrack; 8],
    base: *mut ItemArray<Animation>,
    add: *mut ItemArray<Animation>,
    add2: *mut ItemArray<Animation>,
    neck: *mut AnimationNeck,
    biped_ik: *mut BipedIK,
    bounding_box: BoundingBox,
    field_2d8: usize
}

#[repr(C)]
pub struct AnimControllerTrack {
    type_: u16,
    flags: u32,
    number: u32,
    slot: [AnimControllerSlot; 4],
    slot_mask: u16,
    slot_current: u16
}

#[repr(C)]
pub struct AnimControllerSlot {
    sequence: u16,
    flags: u16,
    weight: f32,
    blend_time: f32,
    duration: f32
}
