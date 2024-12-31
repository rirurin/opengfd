#![allow(dead_code)]
use std::ptr::NonNull;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeafCategory {   
    Dummy = 0,
    Particle = 1,
    FlashPolygon = 2,
    CirclePolygon = 3,
    LightningPolygon = 4,
    TrajectoryPolygon = 5,
    WindPolygon = 6,
    Model = 7,
    SoulPolygon = 8,
    BoardPolygon = 9,
    ObjectParticles = 10,
    GlitterPolygon = 11,
    BrightLightPolygon = 12,
    DirectionalParticles = 13,
    Camera = 14,
    Light = 15,
    PostEffect = 16,
    Helper = 17
}

#[repr(C)]
#[derive(Debug)]
pub struct Part {
    category: LeafCategory,
    type_: u32,
    handle: NonNull<u8>
}
