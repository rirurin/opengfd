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
    graphics::resources::ResBuffer,
    utility::misc::{ Fade, Range, RGBA }
};
use bitflags::bitflags;
use glam::{ Vec3A, Mat4 };
use crate::graphics::curve::Curve2;
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThunderPolygonType
{
    Bar = 1,
    Sphere
}

impl TryFrom<u32> for ThunderPolygonType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Bar),
            2 => Ok(Self::Sphere),
            v => Err(EplError::InvalidThunderPolygonType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonThunder<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: ThunderPolygonType,
    number: u32,
    color: RGBA,
    time: f32,
    refresh: f32,
    seed: u32,
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
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for PolygonThunder<AObject>
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
impl<AObject> PolygonThunder<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            ThunderPolygonType::Bar => EPLParameter::<BarParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            ThunderPolygonType::Sphere => EPLParameter::<SphereParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
#[derive(Debug)]
pub struct BasicParams {
    flags: BasicParamFlags,
    life: f32,
    number: u32,
    count: Range,
    fade: Fade,
    blend: u32,
    split: u32,
    wave: Range,
    wave_height: Range,
    uneven: Range,
    both_ends: Fade,
    width_core: f32,
    width_edge: f32,
    rgba_core: RGBA,
    rgba_border: RGBA, 
    rgba_edge: RGBA,
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
        self.flags = BasicParamFlags::from_bits_truncate(stream.read_u32()?);
        self.life = stream.read_f32()?;
        self.number = stream.read_u32()?;
        self.blend = stream.read_u32()?;
        self.split = stream.read_u32()?;
        self.width_core = stream.read_f32()?;
        self.width_edge = stream.read_f32()?;
        self.rgba_core = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_border = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_edge = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.seed = stream.read_u32()?;
        self.count = Range::stream_read(stream, &mut ())?.into_raw();
        self.wave = Range::stream_read(stream, &mut ())?.into_raw();
        self.wave_height = Range::stream_read(stream, &mut ())?.into_raw();
        self.uneven = Range::stream_read(stream, &mut ())?.into_raw();
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        self.both_ends = Fade::stream_read(stream, &mut ())?.into_raw();
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
pub struct BarParams {
    basic: BasicParams,
    length: Range 
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for BarParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: BarParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl BarParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.length = Range::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct BarParts {
    basic: BasicParts,
    wave: f32,
    wave_height: f32,
    uneven: f32,
    length: f32,
    seed: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SphereParams {
    basic: BasicParams,
    radius: Curve2,
    round: Range,
    rotate_y_speed: Range,
    rotate_y_accele: f32,
    rotate_y_type: u32,
    rotate_z_speed: Range,
    rotate_z_accele: f32,
    rotate_z_type: u32,
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
        self.round = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_y_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_y_accele = stream.read_f32()?;
        self.rotate_y_type = stream.read_u32()?;
        self.rotate_z_speed = Range::stream_read(stream, &mut ())?.into_raw();
        self.rotate_z_accele = stream.read_f32()?;
        self.rotate_z_type = stream.read_u32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct SphereParts {
    basic: BasicParts,
    axis: f32,
    round: f32,
    rotate_y: f32,
    rotate_y_speed: f32,
    rotate_y_accele: f32,
    rotate_z: f32,
    rotate_z_speed: f32,
    rotate_z_accele: f32,
    radius_begin: f32,
    radius_end: f32,
    radian_y: f32,
    radian_z: f32,
    wave: f32,
    wave_height: f32,
    uneven: f32,
    seed: u32,
    refresh: f32,
}
