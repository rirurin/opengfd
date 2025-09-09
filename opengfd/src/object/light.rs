use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    kernel::{
        allocator::GfdAllocator,
        version::GfdVersion
    },
    object::object::{CastFromObject, Object, ObjectId},
    utility::{
        misc::RGBAFloat,
        reference::{ Reference, GfdRcType },
    }
};
use glam::Vec3A;
use std::{
    error::Error,
    fmt::Debug,
    io::{Read, Seek, Write},
    ptr::NonNull
};
use std::fmt::Formatter;
use opengfd_proc::GfdRcAuto;

#[cfg(feature = "serialize")]
use crate::utility::stream::{
    DeserializationHeap,
    DeserializationStrategy,
    GfdSerialize,
    SerializationSingleAllocator,
    Stream,
    StreamError,
    StreamIODevice
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LightType
{
    Directional = 1,
    Point = 2,
    Spot = 3,
}

impl TryFrom<u32> for LightType {
    type Error = StreamError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LightType::Directional),
            2 => Ok(LightType::Point),
            3 => Ok(LightType::Spot),
            _ => Err(StreamError::InvalidEnumValue)
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LightFlags : u32 {
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

#[allow(non_snake_case)]
#[repr(C)]
// #[derive(Debug, GfdRcAuto)]
#[derive(GfdRcAuto)]
pub struct Light<A = GfdAllocator>
where A: Allocator + Clone {
    super_: Object<A>,
    position: Vec3A,
    direction: Vec3A,
    ambient: RGBAFloat,
    diffuse: RGBAFloat,
    specular: RGBAFloat,
    attenuation: LightAttenuation,
    flags: LightFlags,
    type_: LightType,
    scale: f32,
    alpha: f32,
    toon: f32,
    ref_: Reference,
    state: Option<NonNull<LightState>>,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _allocator: A
}

type DynRes<T> = Result<T, Box<dyn Error>>;

impl<A> CastFromObject for Light<A>
where A: Allocator + Clone
{
    const OBJECT_ID: ObjectId = ObjectId::Light;
}

impl<A> Debug for Light<A>
where A: Allocator + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut printf = format!("GfdLight {{ type: {:?}, flags: {:?}", self.type_, self.flags);
        match self.type_{
            LightType::Directional => printf.push_str(" TODO: Directional"),
            LightType::Point => printf.push_str("TODO: Point"),
            LightType::Spot => printf.push_str("TODO: Spot"),
        };
        printf.push_str(" }}");
        write!(f, "{}", printf)
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Light<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> DynRes<DeserializationHeap<Self, AObject>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.ref_ = Reference::new();
        this.stream_read_inner(stream)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Light<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> DynRes<()>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug {
        self.flags = stream
            .has_feature(GfdVersion::LightAddFlags)
            .map_or::<DynRes<LightFlags>, _>(
                Ok(LightFlags::empty()),
                |_| Ok(LightFlags::from_bits_truncate(stream.read_u32()?))
            )?;
        self.type_ = stream.read_u32()?.try_into()?;
        self.ambient = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.diffuse = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.specular = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        match self.type_ {
            LightType::Directional => {
                self.direction = Vec3A::stream_read(stream, &mut ())?.into_raw();
            },
            LightType::Point => {
                self.direction = Vec3A::stream_read(stream, &mut ())?.into_raw();
                self.attenuation.theta = stream.read_f32()?;
                self.attenuation.phi = stream.read_f32()?;
            },
            LightType::Spot => {
                self.position = Vec3A::stream_read(stream, &mut ())?.into_raw();
                match self.flags.contains(LightFlags::Flag1) {
                    true => {
                        self.attenuation.ds =  stream.read_f32()?;
                        self.attenuation.de =  stream.read_f32()?;
                    },
                    false => {
                        self.attenuation.kc =  stream.read_f32()?;
                        self.attenuation.kl =  stream.read_f32()?;
                        self.attenuation.kq =  stream.read_f32()?;
                    }
                }
            }
        }
        self.alpha = stream.has_feature(GfdVersion::LightAddAlpha).map_or(Ok(0.), |_| stream.read_f32())?;
        self.toon = stream.has_feature(GfdVersion::LightAddToonInfluence).map_or(Ok(1.), |_| stream.read_f32())?;
        Ok(())
    }
}

impl<A> Light<A>
where A: Allocator + Clone
{
    // Original function: gfdLightSetAlpha
    fn set_alpha(&mut self, value: f32) {
        self.alpha = value;
    }
    // Original function: gfdLightSetAmbient
    fn set_ambient(&mut self, value: RGBAFloat) {
        self.ambient = value;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LightAttenuation {
    kc: f32,
    kl: f32,
    kq: f32,
    ds: f32,
    de: f32,
    theta: f32,
    phi: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct LightContainer {
    array: [*mut Light; 3],
    ref_: crate::utility::reference::Reference
}

#[repr(C)]
#[derive(Debug)]
pub struct LightState {
    position: Vec3A,
    direction: Vec3A,
    diffuse: RGBAFloat,
    power: f32,
    specular_power: f32,
    toon_influence: f32,
    ds: f32,
    de: f32,
    theta: f32,
    phi: f32,
    flags: u32
}