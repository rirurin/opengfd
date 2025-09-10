#![allow(dead_code)]

use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::object::epl::{EplError, EplLeafFlags};
use crate::utility::misc::{Fade, Range};
use crate::utility::name::{Name, NameSerializationContext, NameSerializationHash};
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

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

impl TryFrom<u32> for LeafCategory {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dummy),
            1 => Ok(Self::Particle),
            2 => Ok(Self::FlashPolygon),
            3 => Ok(Self::CirclePolygon),
            4 => Ok(Self::LightningPolygon),
            5 => Ok(Self::TrajectoryPolygon),
            6 => Ok(Self::WindPolygon),
            7 => Ok(Self::Model),
            8 => Ok(Self::SoulPolygon),
            9 => Ok(Self::BoardPolygon),
            10 => Ok(Self::ObjectParticles),
            11 => Ok(Self::GlitterPolygon),
            12 => Ok(Self::BrightLightPolygon),
            13 => Ok(Self::DirectionalParticles),
            14 => Ok(Self::Camera),
            15 => Ok(Self::Light),
            16 => Ok(Self::PostEffect),
            17 => Ok(Self::Helper),
            v => Err(EplError::InvalidLeafCategory(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Part<A = GfdAllocator>
where A: Allocator + Clone {
    category: LeafCategory,
    type_: u32,
    handle: Option<NonNull<u8>>,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Part<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Part<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        self.category = stream.read_u32()?.try_into()?;
        self.type_ = stream.read_u32()?;
        self.handle = match self.category {
            _ => None
        };
        Ok(())
    }
}