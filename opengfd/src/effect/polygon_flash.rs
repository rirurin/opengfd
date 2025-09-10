use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{
        curve::{ Curve2, Curve4 },
        resources::ResBuffer
    },
    utility::misc::{ Fade, Range, RGB, RGBA }
};
use glam::{ Vec3A, Mat4 };
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlashPolygonType {
    Radiation = 1,
    Explosion,
    Ring,
    Scatter,
    Cylinder
}

impl TryFrom<u32> for FlashPolygonType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Radiation),
            2 => Ok(Self::Explosion),
            3 => Ok(Self::Ring),
            4 => Ok(Self::Scatter),
            5 => Ok(Self::Cylinder),
            v => Err(EplError::InvalidFlashPolygonType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonFlash<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: FlashPolygonType,
    number: u32,
    color: RGBA,
    time: f32,
    refresh: f32,
    dirty: u32,
    seed: u32,
    base: u32,
    material: Option<NonNull<EPLMaterial<A>>>,
    vertex_buffer: *mut u8,
    command: [Option<NonNull<ResBuffer>>; 2],
    job_data: *mut u8,
    parts: *mut u8,
    params: NonNull<EPLParameter<BasicParams, A>>,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for PolygonFlash<AObject>
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
impl<AObject> PolygonFlash<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            FlashPolygonType::Radiation => EPLParameter::<RadiateParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            FlashPolygonType::Explosion => EPLParameter::<SparkParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            FlashPolygonType::Ring => EPLParameter::<RingParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            FlashPolygonType::Scatter => EPLParameter::<ScatterParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            FlashPolygonType::Cylinder => EPLParameter::<CylinderParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>
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
    alpha: f32,
    fade: Fade,
    random: f32,
    blend: u32,
    scale: Curve2,
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
        self.flag = BasicParamFlags::from_bits_truncate(stream.read_u32()?);
        self.life = stream.read_f32()?;
        self.number = stream.read_u32()?;
        self.alpha = stream.read_f32()?;
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
pub struct RadiateParams {
    basic: BasicParams,
    rgba_in: RGBA,
    rgba_out: RGBA,
    width_in: Range,
    width_out: Range,
    length: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for RadiateParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: RadiateParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl RadiateParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.rgba_in = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_out = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.width_in = Range::stream_read(stream, &mut ())?.into_raw();
        self.width_out = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RadiateParts {
    basic: BasicParts,
    width_in: f32,
    width_out: f32,
    length: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SparkParams {
    basic: BasicParams,
    radius: Range,
    rgba_in: RGBA,
    rgba_out: RGBA,
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
        self.rgba_in = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_out = RGBA::stream_read(stream, &mut ())?.into_raw();
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
    width: f32,
    length: f32,
    speed: f32,
    radian: f32,
    position: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct RingParams {
    basic: BasicParams,
    radius: Curve2,
    spread: Curve2,
    rgba_in: RGBA,
    rgba_out: RGBA,
    width_in: Range,
    width_out: Range,
    length: Range,
    rotate_speed: Range,
    rotate_accele: f32,
    rotate_type: u32,
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
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.spread = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.rgba_in = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_out = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.width_in = Range::stream_read(stream, &mut ())?.into_raw();
        self.width_out = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        // Remove color data
        // if stream.has_feature(GfdVersion::EplPolygonFlashRingSpread).is_none() {
        // }
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RingParts {
    basic: BasicParts,
    width_in: f32,
    width_out: f32,
    length: f32,
    rotate: f32,
    rotate_speed: f32,
    rotate_accele: f32,
    spread_start: f32,
    spread_end: f32,
    radius: f32,
    spread: f32,
    radian: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ScatterParams {
    basic: BasicParams,
    radius: Curve2,
    spread: Curve2,
    rgba_in: RGBA,
    rgba_out: RGBA,
    width_in: Range,
    width_out: Range,
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
        self.rgba_in = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_out = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.width_in = Range::stream_read(stream, &mut ())?.into_raw();
        self.width_out = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.accele = stream.read_f32()?;
        // Remove color data
        // if stream.has_feature(GfdVersion::EplPolygonFlashScatterSpread).is_none() {
        // }
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct ScatterParts {
    basic: BasicParts,
    width_in: f32,
    width_out: f32,
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
    radius: Curve2,
    rgb: Curve4,
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
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.rgb = match stream.has_feature(GfdVersion::EplBehaviorUseCurve4ForLife) {
            Some(_) => Ok::<Curve4, Box<dyn Error>>(Curve4::stream_read(stream, &mut ())?.into_raw()),
            None => Ok(Curve2::stream_read(stream, &mut ())?.into_raw().into())
        }?;
        self.width = Range::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        self.speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.accele = stream.read_f32()?;
        self.rotate_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_accele = stream.read_f32()?;
        self.rotate_type = stream.read_u32()?;
        // Remove radius data
        // if stream.has_feature(GfdVersion::EplPolygonFlashCylinderSpread).is_none() {
        // }
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
    rgb: RGB,
}
