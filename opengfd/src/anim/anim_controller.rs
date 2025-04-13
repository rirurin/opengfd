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
use std::ptr::NonNull;

bitflags! {
    pub struct AnimationPackFlags: u32 {
        const Pause   = 1 << 0;
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

const ANIM_TRACK_COUNT: usize = 8;


#[repr(C)]
pub struct AnimController {
    flags: AnimationPackFlags,
    interpolator: NonNull<AnimInterpolator>,
    effector: NonNull<AnimEffector>,
    tracks: [AnimControllerTrack; ANIM_TRACK_COUNT],
    base: *mut ItemArray<Animation>,
    add: *mut ItemArray<Animation>,
    add2: *mut ItemArray<Animation>,
    neck: *mut AnimationNeck,
    biped_ik: *mut BipedIK,
    bounding_box: BoundingBox,
    field_2d8: usize
}

impl AnimController {
    /// Original function: gfdAnimControllerCheckPause
    pub fn check_pause(&self) -> bool { self.flags.contains(AnimationPackFlags::Pause) }
    /// Original function: gfdAnimControllerGetAdditionalAnimationCount
    pub fn get_add_anim_count(&self) -> usize {
        let arr: &ItemArray<Animation> = unsafe { &*self.add };
        arr.len() as usize
    }
    pub fn get_add2_anim_count(&self) -> usize {
        let arr: &ItemArray<Animation> = unsafe { &*self.add2 };
        arr.len() as usize
    }
    fn get_anim_track(&self, index: usize) -> Option<&AnimControllerTrack> {
        match index < ANIM_TRACK_COUNT {
            true => Some(&self.tracks[index]),
            false => None
        }
    }
    /// Original function: gfdAnimControllerGetAnimationNumber
    pub fn get_anim_num(&self, index: usize) -> u32 {
        match self.get_anim_track(index) {
            Some(v) => v.number,
            None => u32::MAX
        }
    }
    /// Original function: gfdAnimControllerGetAnimationType
    pub fn get_anim_type(&self, index: usize) -> u16 {
        match self.get_anim_track(index) {
            Some(v) => v.type_,
            None => 0
        }
    }
    /// Original function: gfdAnimControllerGetBaseAnimationCount
    pub fn get_base_anim_count(&self) -> usize {
        let arr: &ItemArray<Animation> = unsafe { &*self.base };
        arr.len() as usize
    }

    fn get_sequence_index(&self, index: usize) -> usize {
        if let Some(seq) = self.get_anim_track(index) {
            if seq.slot_mask >> (seq.slot_current & 0x3f) & 1 != 0 {
                seq.slot[seq.slot_current as usize].sequence as usize
            } else { usize::MAX }
        } else { usize::MAX }
    }
    /// Original function: gfdAnimControllerGetCurrentTime
    pub fn get_current_time(&self, index: usize) -> f32 {
        match self.get_sequence_index(index) {
            usize::MAX => 0.,
            v => self.get_interpolator().get_current_time(v),
        }
    }
    /// Original function: gfdAnimControllerGetDuration
    pub fn get_duration(&self, index: usize) -> f32 {
        match self.get_sequence_index(index) {
            usize::MAX => 0.,
            v => self.get_interpolator().get_duration(v),
        }
    }
    /// Original function: gfdAnimControllerGetSpeed
    pub fn get_speed(&self, index: usize) -> f32 {
        match self.get_sequence_index(index) {
            usize::MAX => 0.,
            v => self.get_interpolator().get_frequency(v),
        }
    }
    /// Original function: gfdAnimControllerGetWeight
    pub fn get_weight(&self, index: usize) -> f32 {
        match self.get_sequence_index(index) {
            usize::MAX => 0.,
            v => self.get_interpolator().get_weight(v),
        }
    }
    /// Original function: gfdAnimControllerGetInterpolator
    pub fn get_interpolator(&self) -> &AnimInterpolator {
        unsafe { self.interpolator.as_ref() }
    }
    pub fn get_interpolator_mut(&mut self) -> &mut AnimInterpolator {
        unsafe { self.interpolator.as_mut() }
    }
    /// Original function: gfdAnimControllerGetEffector
    pub fn get_effector(&self) -> &AnimEffector {
        unsafe { self.effector.as_ref() }
    }
    pub fn get_effector_mut(&mut self) -> &mut AnimEffector {
        unsafe { self.effector.as_mut() }
    }
    /// Original function: gfdAnimControllerPause
    pub fn pause(&mut self) {
        for track in &self.tracks {
            for slot in 0..ANIM_CONTROL_TRACK_SLOTS {
                if track.slot_mask & 1 << slot != 0 {
                    let seq_idx = track.slot[slot].sequence as usize;
                    // self.get_interpolator_mut().pause(seq_idx);
                    let interp = unsafe { self.interpolator.as_mut() };
                    interp.pause(seq_idx);
                }
            }
        }
        self.flags |= AnimationPackFlags::Pause;
    }
}

const ANIM_CONTROL_TRACK_SLOTS: usize = 4;

#[repr(C)]
pub struct AnimControllerTrack {
    type_: u16,
    flags: u32,
    number: u32,
    slot: [AnimControllerSlot; ANIM_CONTROL_TRACK_SLOTS],
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
