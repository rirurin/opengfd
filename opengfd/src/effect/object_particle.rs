use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use crate::{
    effect::{ 
        behavior::Behavior,
        particle::ParticleEmitterType,
        resources::{ EPLParameter, EPLMaterial }
    },
    object::node::Node,
    utility::misc::RGBA
};
use glam::{Vec3A, Quat, Vec3};
use crate::kernel::allocator::GfdAllocator;
use bitflags::bitflags;
use crate::kernel::version::GfdVersion;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(C)]
#[derive(Debug)]
pub struct ObjectParticle<A = GfdAllocator>
where A: Allocator + Clone {
    type_: ParticleEmitterType,
    number: u32,
    scale: f32,
    color: RGBA,
    behavior: Option<NonNull<Behavior<A>>>,
    material: Option<NonNull<EPLMaterial<A>>>,
    parts: *mut u8,
    params: NonNull<EPLParameter<BasicParams, A>>,
    dirty: u32,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for ObjectParticle<AObject>
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
impl<AObject> ObjectParticle<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        if stream.has_feature(GfdVersion::EplBehaviorBlurParams).is_none() {
            stream.seek(SeekFrom::Current(size_of::<u32>() as i64))?;
        }
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            ParticleEmitterType::Smoke => EPLParameter::<SmokeParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Explosion => EPLParameter::<SparkParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Spiral => EPLParameter::<SpiralParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Ball => EPLParameter::<SphereParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::Circle => EPLParameter::<RingParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ParticleEmitterType::StraightLine => EPLParameter::<LineParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
        })};
        Ok(())
    }

    pub fn get_params(&self) -> &EPLParameter<BasicParams, AObject> {
        unsafe { self.params.as_ref() }
    }

    pub fn get_params_mut(&mut self) -> &mut EPLParameter<BasicParams, AObject> {
        unsafe { self.params.as_mut() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BasicParts {
    rotate0: Quat,
    axis: Vec3A,
    node: *mut Node,
    handle: *mut u8
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplObjectParticleFlags : u32 {
        const Flag0 = 1 << 0;
        const Flag1 = 1 << 1;
        const Flag2 = 1 << 2;
        const Flag3 = 1 << 3;
        const Flag4 = 1 << 4;
        const Flag5 = 1 << 5;
        const Flag6 = 1 << 6;
        const Flag7 = 1 << 7;
        const Flag8 = 1 << 8;
        const Flag9 = 1 << 9;
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
#[derive(Debug)]
pub struct BasicParams {
    flag: EplObjectParticleFlags,
    number: u32,
    axis: Vec3
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for BasicParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: BasicParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl BasicParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.flag = EplObjectParticleFlags::from_bits_truncate(stream.read_u32()?);
        self.number = stream.read_u32()?;
        self.axis = Vec3::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SmokeParams {
    basic: BasicParams,
    behavior: super::behavior::SmokeParams
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SmokeParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SmokeParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SmokeParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.behavior = super::behavior::SmokeParams::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    behavior: super::behavior::SparkParams
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SparkParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SparkParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SparkParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.behavior = super::behavior::SparkParams::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SpiralParams {
    basic: BasicParams,
    behavior: super::behavior::SpiralParams
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SpiralParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SpiralParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SpiralParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.behavior = super::behavior::SpiralParams::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SphereParams {
    basic: BasicParams,
    behavior: super::behavior::SphereParams
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SphereParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SphereParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SphereParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.behavior = super::behavior::SphereParams::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RingParams {
    basic: BasicParams,
    behavior: super::behavior::RingParams
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for RingParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: RingParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl RingParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.behavior = super::behavior::RingParams::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct LineParams {
    basic: BasicParams,
    behavior: super::behavior::LineParams
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for LineParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: LineParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl LineParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.behavior = super::behavior::LineParams::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}