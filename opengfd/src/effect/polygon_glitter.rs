use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
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
use glam::{Vec3A, Mat4, Vec2};
use crate::kernel::allocator::GfdAllocator;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};
use bitflags::bitflags;
use crate::kernel::version::GfdVersion;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GlitterPolygonType
{
    Explosion = 1,
    Splash,
    Cylinder,
    Wall
}

impl TryFrom<u32> for GlitterPolygonType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Explosion),
            2 => Ok(Self::Splash),
            3 => Ok(Self::Cylinder),
            4 => Ok(Self::Wall),
            v => Err(EplError::InvalidFlashPolygonType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonGlitter<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: GlitterPolygonType,
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
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for PolygonGlitter<AObject>
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
impl<AObject> PolygonGlitter<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            GlitterPolygonType::Explosion => EPLParameter::<SparkParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            GlitterPolygonType::Splash => EPLParameter::<ScatterParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            GlitterPolygonType::Cylinder => EPLParameter::<CylinderParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            GlitterPolygonType::Wall => EPLParameter::<WallParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
    time: f32,
    rgba_in: RGBA,
    rgba_out: RGBA
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplPolygonGlitterFlags : u32 {
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
    flag: EplPolygonGlitterFlags,
    life: f32,
    number: u32,
    count: Range,
    fade: Fade,
    random: f32,
    blend: u32,
    rgba_in: Curve4,
    rgba_out: Curve4,
    scale: Curve2,
    repeat_tex_v: f32,
    move_tex_v: f32,
    adjust: Adjustment,
    seed: u32,
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
        self.flag = EplPolygonGlitterFlags::from_bits_truncate(stream.read_u32()?);
        self.life = stream.read_f32()?;
        self.number = stream.read_u32()?;
        self.blend = stream.read_u32()?;
        self.seed = stream.read_u32()?;
        self.count = Range::stream_read(stream, &mut ())?.into_raw();
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        self.scale = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.random = stream
            .has_feature(GfdVersion::EplBehaviorRandomColor)
            .map_or::<Result<f32, Box<dyn Error>>, _>(
                Ok(0.),
                |_| Ok(stream.read_f32()?)
            )?;
        self.rgba_in = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.rgba_out = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.repeat_tex_v = stream.read_f32()?;
        self.move_tex_v = stream.read_f32()?;
        self.adjust = Adjustment::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Range,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
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
        self.radius = Range::stream_read(stream, &mut ())?.into_raw();
        self.width = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.accele = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SparkParts {
    basic: BasicParts,
    radius: f32,
    width: f32,
    length: f32,
    speed: f32,
    radian: f32,
    position: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ScatterParams {
    basic: BasicParams,
    radius: Curve2,
    spread: Curve2,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for ScatterParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: ScatterParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl ScatterParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.spread = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.accele = stream.read_f32()?;
        if stream.has_feature(GfdVersion::EplPolygonFlashRingSpread).is_none() {
            unsafe { self.spread.reset_as::<Vec2>() };
            self.spread.set_start_point(Vec2::ZERO);
            self.spread.set_end_point(Vec2::ZERO);
        }
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct ScatterParts {
    basic: BasicParts,
    width: f32,
    length: f32,
    speed: f32,
    radian: f32,
    radius: f32,
    spread_start: f32,
    spread_end: f32,
    spread: f32,
    move_: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct CylinderParams {
    basic: BasicParams,
    radius: Range,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for CylinderParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: CylinderParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl CylinderParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius = Range::stream_read(stream, &mut ())?.into_raw();
        self.width = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.accele = stream.read_f32()?;
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct CylinderParts {
    basic: BasicParts,
    width: f32,
    length: f32,
    speed: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radius: f32,
    radius_start: f32,
    radius_end: f32,
    radian: f32,
    move_: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct WallParams {
    basic: BasicParams,
    distance: Curve2,
    width: Range,
    length: Range,
    speed: Range,
    accele: f32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for WallParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: WallParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl WallParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.distance = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.accele = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct WallParts {
    basic: BasicParts,
    width: f32,
    length: f32,
    speed: f32,
    distance: f32,
    distance_start: f32,
    distance_end: f32,
    move_: f32,
}
