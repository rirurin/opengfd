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
use std::ptr::NonNull;

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

#[repr(C)]
pub struct AnimationSequence {
    flags: u32,
    time: f32,
    current_time: f32,
    animation: *mut Animation,
    type_: u32,
    weight: f32,
    frequency: f32
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
    sequence: [AnimationSequence; 32],
    sequece_mask: u32,
    nodes: AnimKeyController<KeyNode, 0>,
    materials: AnimKeyController<KeyMaterial>,
    shapes: AnimKeyController<KeyShape>,
    cameras: AnimKeyController<KeyCamera>,
    lights: AnimKeyController<KeyLight>,
    ref_: Reference,
    dirty: u32
}
