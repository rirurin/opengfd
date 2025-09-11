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
        curve::Curve4,
        resources::ResBuffer
    },
    utility::misc::{ Fade, Range, RGBA }
};
use glam::{ Vec2, Vec3A, Mat4 };
use bitflags::bitflags;
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlashPolygonType {
    Square = 1,
    Oblong
}

impl TryFrom<u32> for FlashPolygonType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Square),
            2 => Ok(Self::Oblong),
            v => Err(EplError::InvalidBoardPolygonType(v))
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplPolygonBoardFlags : u32 {
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
pub struct PolygonBoard<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: FlashPolygonType,
    color: RGBA,
    time: f32,
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
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for PolygonBoard<AObject>
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
impl<AObject> PolygonBoard<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            FlashPolygonType::Square => EPLParameter::<SquareParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            FlashPolygonType::Oblong => EPLParameter::<OblongParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
    flag: EplPolygonBoardFlags,
    life: f32, // anim_length
    fade: Fade,
    blend: u32, // layer_mode
    adjust: Adjustment,
    seed: u32
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
        self.flag = EplPolygonBoardFlags::from_bits_truncate(stream.read_u32()?);
        self.life = stream.read_f32()?;
        self.blend = stream.read_u32()?;
        self.seed = stream.read_u32()?;
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        self.adjust = stream
            .has_feature(GfdVersion::EplBehaviorAdjustmentParams)
            .map_or::<Result<Adjustment, Box<dyn Error>>, _>(
                Ok(Adjustment::default()),
                |_| Ok(Adjustment::stream_read(stream, &mut ())?.into_raw())
            )?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SquareParts {
    basic: BasicParts,
    rotate_start: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SquareParams {
    basic: BasicParams,
    pivot: Vec2,
    length: Curve4,
    rgb: Curve4,
    alpha: f32,
    rotate_start: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SquareParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SquareParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SquareParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.length = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.rgb = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        self.rotate_start = stream
            .has_feature(GfdVersion::EplPolygonFlashRingSpread)
            .map_or::<Result<Range, Box<dyn Error>>, _>(
                Ok(Range::default()),
                |_| Ok(Range::stream_read(stream, &mut ())?.into_raw())
            )?;
        self.pivot = stream
            .has_feature(GfdVersion::EplPolygonBoardPivot)
            .map_or::<Result<Vec2, Box<dyn Error>>, _>(
                Ok(Vec2::default()),
                |_| Ok(Vec2::stream_read(stream, &mut ())?.into_raw())
            )?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct OblongParts {
    basic: BasicParts,
    rotate_start: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct OblongParams {
    basic: BasicParams,
    pivot: Vec2,
    width: Curve4,
    height: Curve4,
    rgb: Curve4,
    alpha: f32,
    rotate_start: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for OblongParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: OblongParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl OblongParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.width = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.height = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.rgb = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.alpha = stream.read_f32()?;
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        self.rotate_start = Range::stream_read(stream, &mut ())?.into_raw();
        self.pivot = Vec2::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}