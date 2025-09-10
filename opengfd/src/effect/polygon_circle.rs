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
    utility::misc::{ Fade, RGBA }
};
use glam::{Vec3A, Mat4, Vec2};
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(u32)]
#[derive(Debug)]
pub enum CirclePolygonType {
    Ring = 1,
    Trajectory,
    Fill,
    Hoop
}

impl TryFrom<u32> for CirclePolygonType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ring),
            2 => Ok(Self::Trajectory),
            3 => Ok(Self::Fill),
            4 => Ok(Self::Hoop),
            v => Err(EplError::InvalidCirclePolygonType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PolygonCircle<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: CirclePolygonType,
    color: RGBA,
    time: f32,
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
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for PolygonCircle<AObject>
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
impl<AObject> PolygonCircle<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            CirclePolygonType::Ring => EPLParameter::<RingParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            CirclePolygonType::Trajectory => EPLParameter::<TrackParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            CirclePolygonType::Fill => EPLParameter::<FillParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            CirclePolygonType::Hoop => EPLParameter::<HoopParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
    fade: Fade,
    blend: u32,
    adjust: Adjustment,
    seed: u32,
    split: u32
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
        self.blend = stream.read_u32()?;
        self.seed = stream.read_u32()?;
        self.split = stream.read_u32()?;
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

#[repr(C)]
#[derive(Debug)]
pub struct RingParams {
    basic: BasicParams,
    occurrence: f32,
    radius: Curve2,
    width: Curve2,
    width_ratio: Fade,
    rgba_in: RGBA,
    rgba_center: RGBA,
    rgba_out: RGBA,
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
        self.occurrence = stream.read_f32()?;
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width_ratio = Fade::stream_read(stream, &mut ())?.into_raw();
        self.rgba_in = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_center = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rgba_out = RGBA::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RingParts {
    basic: BasicParts
}

#[repr(C)]
#[derive(Debug)]
pub struct TrackParams {
    basic: BasicParams,
    radius: f32,
    width_ratio: f32,
    rgba_in: Curve4,
    rgba_out: Curve4,
    speed: f32,
    accele: f32,
    repeat_tex_u: f32,
    move_tex_v: f32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for TrackParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: TrackParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl TrackParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius = stream.read_f32()?;
        self.width_ratio = stream.read_f32()?;
        match stream.has_feature(GfdVersion::EplPolygonCircleTrackRGBCurve) {
            Some(_) => {
                self.rgba_in = Curve4::stream_read(stream, &mut ())?.into_raw();
                self.rgba_out = Curve4::stream_read(stream, &mut ())?.into_raw();
            },
            None => {
                unsafe { self.rgba_in.reset_as::<RGBA>() };
                unsafe { self.rgba_out.reset_as::<RGBA>() };

                let rgba_in = RGBA::stream_read(stream, &mut ())?.into_raw();
                self.rgba_in.set_all_targets(rgba_in)?;
                let rgba_out = RGBA::stream_read(stream, &mut ())?.into_raw();
                self.rgba_out.set_all_targets(rgba_out)?;

                self.rgba_in.set_start_point(Vec2::new(0.3333, 0.3333));
                self.rgba_in.set_end_point(Vec2::new(0.6667, 0.6667));
                self.rgba_in.rebuild_point_table();

                self.rgba_out.set_start_point(Vec2::new(0.3333, 0.3333));
                self.rgba_out.set_end_point(Vec2::new(0.6667, 0.6667));
                self.rgba_out.rebuild_point_table();
            }
        };
        self.speed = stream.read_f32()?;
        self.accele = stream.read_f32()?;
        self.repeat_tex_u = stream.read_f32()?;
        self.move_tex_v = stream
            .has_feature(GfdVersion::EplPolygonCircleTrackRGBCurve)
            .map_or::<Result<f32, Box<dyn Error>>, _>(
                Ok(0.),
                |_| Ok(stream.read_f32()?)
            )?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct TrackParts {
    basic: BasicParts,
    position: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct FillParams {
    basic: BasicParams,
    occurrence: f32,
    radius: Curve2,
    rgba_in: Curve4,
    rgba_out: Curve4,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for FillParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: FillParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl FillParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.occurrence = stream.read_f32()?;
        self.radius = Curve2::stream_read(stream, &mut ())?.into_raw();
        match stream.has_feature(GfdVersion::EplPolygonCircleTrackRGBCurve) {
            Some(_) => {
                self.rgba_in = Curve4::stream_read(stream, &mut ())?.into_raw();
                self.rgba_out = Curve4::stream_read(stream, &mut ())?.into_raw();
            },
            None => {
                unsafe { self.rgba_in.reset_as::<RGBA>() };
                unsafe { self.rgba_out.reset_as::<RGBA>() };

                let rgba_in = RGBA::stream_read(stream, &mut ())?.into_raw();
                self.rgba_in.set_all_targets(rgba_in)?;
                let rgba_out = RGBA::stream_read(stream, &mut ())?.into_raw();
                self.rgba_out.set_all_targets(rgba_out)?;

                self.rgba_in.set_start_point(Vec2::new(0.3333, 0.3333));
                self.rgba_in.set_end_point(Vec2::new(0.6667, 0.6667));
                self.rgba_in.rebuild_point_table();

                self.rgba_out.set_start_point(Vec2::new(0.3333, 0.3333));
                self.rgba_out.set_end_point(Vec2::new(0.6667, 0.6667));
                self.rgba_out.rebuild_point_table();
            }
        };
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct FillParts {
    basic: BasicParts
}

#[repr(C)]
#[derive(Debug)]
pub struct HoopParams {
    basic: BasicParams,
    occurrence: f32,
    radius: f32,
    speed: f32,
    accele: f32,
    width: Curve2,
    width_ratio: Fade,
    rgba_in: Curve4,
    rgba_center: Curve4,
    rgba_out: Curve4,
    repeat_tex_u: f32,
    move_tex_v: f32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for HoopParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: HoopParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl HoopParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.radius = stream.read_f32()?;
        self.speed = stream.read_f32()?;
        self.accele = stream.read_f32()?;
        self.width = Curve2::stream_read(stream, &mut ())?.into_raw();
        self.width_ratio = Fade::stream_read(stream, &mut ())?.into_raw();
        self.rgba_in = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.rgba_center = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.rgba_out = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.repeat_tex_u = stream.read_f32()?;
        self.move_tex_v = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct HoopParts {
    basic: BasicParts
}