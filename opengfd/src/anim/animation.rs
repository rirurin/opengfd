use bitflags::bitflags;
use crate::{
    anim::key::{ 
        AnimKey, KeyNode, KeyMaterial,
        KeyShape, KeyCamera, KeyLight,
        KeyList
    },
    object::epl::EPL,
    utility::{
        item_array::ItemArray,
        misc::BoundingBox,
        name::Name,
        property::Property,
        reference::Reference
    }
};
use std::{
    error::Error,
    fmt::Display,
    ptr::NonNull
};

bitflags! {
    pub struct AnimationFlags: u32 {
        const Flag0           = 1 << 0;
        const Flag1           = 1 << 1;
        const Flag2           = 1 << 2;
        const Flag3           = 1 << 3;
        const Flag4           = 1 << 4;
        const Flag5           = 1 << 5;
        const Flag6           = 1 << 6;
        const Flag7           = 1 << 7;
        const Flag8           = 1 << 8;
        const Flag9           = 1 << 9;
        const Flag10          = 1 << 10;
        const Flag11          = 1 << 11;
        const Flag12          = 1 << 12;
        const Flag13          = 1 << 13;
        const Flag14          = 1 << 14;
        const Flag15          = 1 << 15;
        const Flag16          = 1 << 16;
        const Flag17          = 1 << 17;
        const Flag18          = 1 << 18;
        const Flag19          = 1 << 19;
        const Flag20          = 1 << 20;
        const Flag21          = 1 << 21;
        const Flag22          = 1 << 22;
        const HasProperties = 1 << 23;
        const Flag24          = 1 << 24;
        const HasSpeed     = 1 << 25;
        const Flag26          = 1 << 26;
        const Flag27          = 1 << 27;
        const Flag28          = 1 << 28;
        const Flag29          = 1 << 29;
        const HasBoundingBox = 1 << 30;
        const Flag31          = 1 << 31;
    }
}

#[repr(C)]
pub struct Animation {
    flags: AnimationFlags,
    duration: f32,
    num_joints: u32,
    joints: Option<NonNull<AnimationJoint>>,
    effect: Option<NonNull<ItemArray<EPL>>>,
    neck: Option<NonNull<AnimationNeck>>,
    bounding_box: *mut BoundingBox,
    translation: AnimationTranslation,
    properties: *mut Property,
    frequency: f32,
    cfb_data: AnimationCFB,
    ref_: Reference
}

impl Animation {
    pub fn has_flags(&self, flags: AnimationFlags) -> bool {
        self.flags.contains(flags)
    }
    pub fn has_any_flags(&self, flags: AnimationFlags) -> bool {
        self.flags.intersects(flags)
    }
    pub fn get_duration(&self) -> f32 {
        self.duration
    }
    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }
}

#[cfg(feature = "cfb_gap")]
#[repr(C)]
pub struct AnimationCFB {
    malloc_fn: Option<fn() -> usize>,
    free_fn: Option<fn(usize) -> ()>,
    field_58: usize,
    field_60: u32,
    field_64: u32,
}

#[repr(C)]
pub struct AnimationNeck {
    anim: [Animation; 4],
    angle: [f32; 4]
}

#[repr(u16)]
pub enum AnimJointType {
    Invalid = 0,
    Node = 1,
    Material = 2,
    Camera = 3,
    Morph = 4
}

#[repr(C)]
pub struct AnimationJoint {
    type_: AnimJointType,
    field_04: u32,
    target_id: u32,
    name: Name,
    num_keys: u32,
    keys: *mut KeyList
}

#[repr(C)]
pub struct AnimationTranslation {
    field_0: u64,
    name: Name,
    keys: *mut KeyList
}

bitflags! {
    pub struct SequenceFlags: u32 {
        const Playing  = 1 << 0;
        const Pause  = 1 << 1;
        const Flag2  = 1 << 2;
        const Flag3  = 1 << 3;
        const Flag4  = 1 << 4;
        const Flag5  = 1 << 5;
        const Flag6  = 1 << 6;
        const Flag7  = 1 << 7;
        const Flag8  = 1 << 8;
        const Flag9  = 1 << 9;
        const Flag10 = 1 << 10;
        const Flag11 = 1 << 11;
        const Flag12 = 1 << 12;
        const Flag13 = 1 << 13;
        const Flag14 = 1 << 14;
        const Flag15 = 1 << 15;
        const Flag16 = 1 << 16;
        const Flag17 = 1 << 17;
        const Flag18 = 1 << 18;
        const Flag19 = 1 << 19;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
        const Flag25 = 1 << 25;
        const Flag26 = 1 << 26;
        const Flag27 = 1 << 27;
        const Flag28 = 1 << 28;
        const Flag29 = 1 << 29;
        const Flag30 = 1 << 30;
        const Flag31 = 1 << 31;
    }
}

#[repr(C)]
pub struct AnimationSequence {
    flags: SequenceFlags,
    time: f32,
    current_time: f32,
    animation: Option<NonNull<Animation>>,
    type_: u32,
    weight: f32,
    frequency: f32
}

impl AnimationSequence {
    pub fn get_anim(&self) -> Option<&Animation> {
        self.animation.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_anim_mut(&mut self) -> Option<&mut Animation> {
        self.animation.map(|mut v| unsafe { v.as_mut() })
    }
}

const ANIM_INTERP_SEQUENCE_COUNT: usize = 0x20;

#[derive(Debug)]
pub struct AnimSequenceIndexError(usize);
impl Error for AnimSequenceIndexError {}
impl Display for AnimSequenceIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Anim Sequence Index {} is out of bounds", self.0)
    }
}

// Pack into u32 alignment so it's aligned correctly inside gfdAnimInterpolator (we use the padding
// field to align onto usize)
#[repr(C, packed(4))]
pub(crate) struct AnimKeyController<
    T: AnimKey, 
    const P: usize = 1
> {
    num: u32,
    padding: [u32; P],
    base: *mut T,
    key: *mut T,
}

#[repr(C)]
pub struct AnimInterpolator {
    sequence: [AnimationSequence; ANIM_INTERP_SEQUENCE_COUNT],
    sequence_mask: u32,
    nodes: AnimKeyController<KeyNode, 0>,
    materials: AnimKeyController<KeyMaterial>,
    shapes: AnimKeyController<KeyShape>,
    cameras: AnimKeyController<KeyCamera>,
    lights: AnimKeyController<KeyLight>,
    ref_: Reference,
    dirty: u32
}

impl AnimInterpolator {
    /// Original function: gfdAnimInterpolatorCheckPause
    pub fn check_pause(&self, index: usize) -> bool {
        match self.get_seq(index) {
            Ok(seq)  => seq.flags.contains(SequenceFlags::Pause),
            Err(_) => false
        }
    }
    /// Original function: gfdAnimInterpolatorGetAnimSequence
    pub fn get_seq(&self, index: usize) -> Result<&AnimationSequence, AnimSequenceIndexError> {
        match index < ANIM_INTERP_SEQUENCE_COUNT {
            true => Ok(&self.sequence[index]),
            false => Err(AnimSequenceIndexError(index))
        }
    }
    pub fn get_seq_mut(&mut self, index: usize) -> Result<&mut AnimationSequence, AnimSequenceIndexError> {
        match index < ANIM_INTERP_SEQUENCE_COUNT {
            true => Ok(&mut self.sequence[index]),
            false => Err(AnimSequenceIndexError(index))
        }
    }
    /// Original function: gfdAnimInterpolatorGetAnimation
    pub fn get_anim(&self, index: usize) -> Option<&Animation> {
        if let Ok(seq) = self.get_seq(index) {
            seq.animation.map(|v| unsafe { v.as_ref() })
        } else { None }
    }
    pub fn get_anim_mut(&mut self, index: usize) -> Option<&mut Animation> {
        if let Ok(seq) = self.get_seq_mut(index) {
            seq.animation.map(|mut v| unsafe { v.as_mut() })
        } else { None }
    }
    /// Original function: gfdAnimInterpolatorGetCurrentTime
    pub fn get_current_time(&self, index: usize) -> f32 {
        match self.get_seq(index) {
            Ok(seq) => seq.current_time,
            Err(_) => 0.
        }
    }
    /// Original function: gfdAnimInterpolatorGetDuration
    pub fn get_duration(&self, index: usize) -> f32 {
        match self.get_anim(index) {
            Some(anim) => anim.get_duration(),
            None => 0.
        }
    }
    /// Original function: gfdAnimInterpolatorGetFrequency
    pub fn get_frequency(&self, index: usize) -> f32 {
        if let Ok(seq) = self.get_seq(index) {
            if let Some(anim) = seq.get_anim() {
                return if anim.has_flags(AnimationFlags::HasSpeed) 
                && anim.get_frequency() > 0. {
                    seq.frequency / anim.get_frequency()
                } else {
                    seq.frequency
                }
            }
        }
        0.
    }
    /// Original function: gfdAnimInterpolatorGetWeight
    pub fn get_weight(&self, index: usize) -> f32 {
        match self.get_seq(index) {
            Ok(seq) => seq.weight,
            Err(_) => 0.
        }
    }
    /// Original function: gfdAnimInterpolatorPause
    pub fn pause(&mut self, index: usize) {
        if let Ok(seq) = self.get_seq_mut(index) {
            seq.flags |= SequenceFlags::Pause;
        }
    }
    /// Original function: gfdAnimInterpolatorResume
    pub fn resume(&mut self, index: usize) {
        if let Ok(seq) = self.get_seq_mut(index) {
            seq.flags &= SequenceFlags::Pause;
        }
    }
    /// Original function: gfdAnimInterpolatorSetCurrentTime
    pub fn set_current_time(&mut self, index: usize, time: f32) {
        if let Ok(seq) = self.get_seq_mut(index) {
            let dur = seq.get_anim().map_or_else(|| 0., |anim| anim.get_duration());
            if time < dur {
                // TODO
            }
            // if seq.flags.contains(SequenceFlags::Playing) && 
            seq.time = 0.;
            seq.current_time = 0.;
        }
    }
    /// Original function: gfdAnimInterpolatorSetFrequency
    pub fn set_frequency(&mut self, index: usize, freq: f32) {
        if let Ok(seq) = self.get_seq_mut(index) {
            if let Some(anim) = seq.get_anim_mut() {
                seq.frequency = if anim.has_flags(AnimationFlags::HasSpeed) 
                && anim.get_frequency() > 0. {
                    freq / anim.get_frequency()
                } else {
                    freq
                }
            }
        }
    }
    /// Original function: gfdAnimInterpolatorSetWeight
    pub fn set_weight(&mut self, index: usize, weight: f32) {
        if let Ok(seq) = self.get_seq_mut(index) {
            seq.weight = weight;
        }
    }
}