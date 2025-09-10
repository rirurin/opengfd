use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use crate::{
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{
        curve::{ Curve2, Curve4 },
        resources::ResBuffer
    },
    utility::misc::{ Fade, Range, RGBA }
};
use glam::{ Vec3, Vec3A, Mat4 };
use crate::effect::particle::ParticleEmitterType;
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};
use bitflags::bitflags;
use crate::object::epl::EplError;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindPolygonType
{
    Spiral = 1,
    Spark,
    Sphere
}

impl TryFrom<u32> for WindPolygonType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Spiral),
            2 => Ok(Self::Spark),
            3 => Ok(Self::Sphere),
            v => Err(EplError::InvalidWindPolygonType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonWind<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: WindPolygonType,
    number: u32,
    color: RGBA,
    time: f32,
    refresh: f32,
    seed: u32,
    base: u32,
    dirty: u32,
    material: Option<NonNull<EPLMaterial<A>>>,
    vertex_buffer: *mut u8,
    command: [Option<NonNull<ResBuffer>>; 2],
    job_data: *mut u8,
    parts: *mut u8,
    params: NonNull<EPLParameter<BasicParams, A>>,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for PolygonWind<AObject>
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
impl<AObject> PolygonWind<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            WindPolygonType::Spiral => EPLParameter::<SpiralParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            WindPolygonType::Spark => EPLParameter::<SparkParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            WindPolygonType::Sphere => EPLParameter::<SphereParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
#[allow(dead_code)]
#[derive(Debug)]
pub struct BasicParts(f32);

#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flag: BasicParamFlags,
    life: f32, 
    number: u32, 
    count: Range, 
    rgb: Curve4, 
    alpha: f32, 
    fade: Fade, 
    random: f32, 
    blend: u32, 
    split: u32, 
    repeat_tex_v: Range, 
    move_tex_v: Range, 
    adjust: Adjustment, 
    seed: u32, 
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BasicParamFlags : u32 {
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
        self.flag = BasicParamFlags::from_bits_truncate(stream.read_u32()?);
        self.life = stream.read_f32()?;
        self.number = stream.read_u32()?;
        self.alpha = stream.read_f32()?;
        self.blend = stream.read_u32()?;
        self.split = stream.read_u32()?;
        self.seed = stream.read_u32()?;
        self.rgb = match stream.has_feature(GfdVersion::EplBehaviorUseCurve4ForLife) {
            Some(_) => Ok::<Curve4, Box<dyn Error>>(Curve4::stream_read(stream, &mut ())?.into_raw()),
            None => Ok(Curve2::stream_read(stream, &mut ())?.into_raw().into())
        }?;
        self.count = Range::stream_read(stream, &mut ())?.into_raw();
        self.repeat_tex_v = Range::stream_read(stream, &mut ())?.into_raw();
        self.move_tex_v = Range::stream_read(stream, &mut ())?.into_raw();
        self.random = stream
            .has_feature(GfdVersion::EplBehaviorRandomColor)
            .map_or::<Result<f32, Box<dyn Error>>, _>(
                Ok(0.),
                |_| Ok(stream.read_f32()?)
            )?;
        self.adjust = stream
            .has_feature(GfdVersion::EplBehaviorAdjustmentParams)
            .map_or::<Result<Adjustment, Box<dyn Error>>, _>(
                Ok(Adjustment::default()),
                |_| Ok(Adjustment::stream_read(stream, &mut ())?.into_raw())
            )?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SpiralParams {
    basic: BasicParams,
    radius: Curve2,
    height: Curve2,
    width: Curve2,
    slant: Range,
    round: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
    both_ends: Fade,
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
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.height = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.slant = Range::stream_read(stream, &mut ())?.into_raw();
        self.round = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        self.both_ends = Fade::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SprialParts {
    basic: BasicParts,
    round: f32,
    slant: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radius_begin: f32,
    radius_end: f32,
    height_begin: f32,
    height_end: f32,
    width_begin: f32,
    width_end: f32,
    repeatv: f32,
    move_v: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Curve2,
    width_ratio: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
    move_tex_v: Range,
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
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width_ratio = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        self.move_tex_v = stream
            .has_feature(GfdVersion::EplPolygonWindMoveTexV)
            .map_or::<Result<Range, Box<dyn Error>>, _>(
                Ok(Range::new(0., 0.)),
                |_| Ok(Range::stream_read(stream, &mut ())?.into_raw())
            )?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SparkParts {
    basic: BasicParts,
    axis: Vec3,
    radius_begin: f32,
    radius_end: f32,
    width_ratio: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    repeat_u: f32,
    move_u: f32,
    move_v: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SphereParams {
    basic: BasicParams,
    radius: Curve2,
    width: Curve2,
    round: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
    both_ends: Fade,
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
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.round = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        self.both_ends = Fade::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SphereParts {
    basic: BasicParts,
    axis: Vec3,
    round: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radius_begin: f32,
    radius_end: f32,
    width_begin: f32,
    width_end: f32,
    repeat_v: f32,
    move_v: f32,
    radian: f32,
}
