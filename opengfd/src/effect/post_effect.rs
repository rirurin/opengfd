use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use crate::{
    effect::{
        // misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    graphics::{ 
        curve::Curve4,
        // resources::ResBuffer
    },
    utility::misc::{ Fade, RGBA }
};
use glam::{ Vec2, Mat4 };
use crate::kernel::allocator::GfdAllocator;
use bitflags::bitflags;
use crate::effect::polygon_flash::RadiateParams;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostEffectType {
    RadialBlur = 1,
    StraightBlur,
    NoiseBlur,
    DistortionBlur,
    FillData,
    LensFlareData,
    ColorCorrectionData,
    MonotoneData,
    #[cfg(feature = "v1-core")]
    LensFlareMake,
    #[cfg(feature = "v2-core")]
    ChromaticAberration,
    #[cfg(feature = "v1-core")]
    MotionBlur,
    #[cfg(feature = "v2-core")]
    ColorCorrectionExcludeToon,
    #[cfg(feature = "v1-core")]
    AfterimageBlur
}

impl TryFrom<u32> for PostEffectType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::RadialBlur),
            2 => Ok(Self::StraightBlur),
            3 => Ok(Self::NoiseBlur),
            4 => Ok(Self::DistortionBlur),
            5 => Ok(Self::FillData),
            6 => Ok(Self::LensFlareData),
            7 => Ok(Self::ColorCorrectionData),
            8 => Ok(Self::MonotoneData),
            #[cfg(feature = "v2-core")]
            9 => Ok(Self::ChromaticAberration),
            #[cfg(feature = "v1-core")]
            9 => Ok(Self::LensFlareMake),
            #[cfg(feature = "v2-core")]
            10 => Ok(Self::ColorCorrectionExcludeToon),
            #[cfg(feature = "v1-core")]
            10 => Ok(Self::MotionBlur),
            #[cfg(feature = "v1-core")]
            11 => Ok(Self::AfterimageBlur),
            v => Err(EplError::InvalidPostEffectType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PostEffect<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    type_: PostEffectType,
    time: f32,
    color: RGBA,
    material: Option<NonNull<EPLMaterial<A>>>,
    params: NonNull<EPLParameter<BasicParams, A>>,
    parts: *mut u8,
    dirty: u32,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for PostEffect<AObject>
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
impl<AObject> PostEffect<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            PostEffectType::RadialBlur => EPLParameter::<RadialBlurParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            PostEffectType::StraightBlur => EPLParameter::<StraightBlurParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            PostEffectType::NoiseBlur => EPLParameter::<NoiseBlurParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            PostEffectType::DistortionBlur => EPLParameter::<DistortBlurParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            PostEffectType::FillData => EPLParameter::<FillParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            PostEffectType::LensFlareData => EPLParameter::<LensFlareParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            PostEffectType::ColorCorrectionData => EPLParameter::<ColorCorrectionParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            PostEffectType::MonotoneData => EPLParameter::<MonotoneParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            #[cfg(feature = "v2-core")]
            PostEffectType::ChromaticAberration => EPLParameter::<ChromaticAberrationParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            #[cfg(feature = "v2-core")]
            PostEffectType::ColorCorrectionExcludeToon => EPLParameter::<ColorCorrectionExcludeToonParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            #[cfg(feature = "v1-core")]
            PostEffectType::LensFlareMake => EPLParameter::<LensFlareMakeParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            #[cfg(feature = "v1-core")]
            PostEffectType::MotionBlur => EPLParameter::<MotionBlurParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            #[cfg(feature = "v1-core")]
            PostEffectType::AfterimageBlur => EPLParameter::<AfterimageBlurParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
    pub struct EplPostEffectFlags : u32 {
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

#[repr(C, align(16))]
#[derive(Debug)]
pub struct BasicParams {
    flag: EplPostEffectFlags,
    life: f32,
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
        self.flag = EplPostEffectFlags::from_bits_truncate(stream.read_u32()?);
        self.life = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RadialBlurParams {
    basic: BasicParams,
    rgba: Curve4, // color_over_time
    blend: u32,
    power: Curve4, // intensity_over_time
    falloff: f32,
    center_x: f32, // blur_focal_point
    center_y: f32,
    ssao_mask: bool,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for RadialBlurParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: RadialBlurParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl RadialBlurParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.rgba = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.blend = stream.read_u32()?;
        self.power = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.falloff = stream.read_f32()?;
        self.center_x = stream.read_f32()?;
        self.center_y = stream.read_f32()?;
        self.ssao_mask = stream
            .has_feature(GfdVersion::EplRadialBlurSSAOMask)
            .map_or::<Result<bool, Box<dyn Error>>, _>(
                Ok(false),
                |_| Ok(stream.read_u8()? != 0)
            )?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct StraightBlurParams {
    basic: BasicParams,
    rgba: Curve4,
    blend: u32,
    power: Curve4,
    direction: f32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for StraightBlurParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: StraightBlurParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl StraightBlurParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.rgba = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.blend = stream.read_u32()?;
        self.power = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.direction = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct NoiseBlurParams {
    basic: BasicParams,
    rgba: Curve4,
    blend: u32,
    power: Curve4,
    scale: Curve4,
    ssao_mask: bool,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for NoiseBlurParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: NoiseBlurParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl NoiseBlurParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.rgba = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.blend = stream.read_u32()?;
        self.power = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.scale = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.ssao_mask = stream
            .has_feature(GfdVersion::EplNoiseBlurSSAOMask)
            .map_or::<Result<bool, Box<dyn Error>>, _>(
                Ok(false),
                |_| Ok(stream.read_u8()? != 0)
            )?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct DistortBlurParams {
    basic: BasicParams,
    rgba: Curve4,
    blend: u32,
    power: [Curve4; 2],
    uv: [Vec2; 2]
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for DistortBlurParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: DistortBlurParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl DistortBlurParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.rgba = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.blend = stream.read_u32()?;
        for i in 0..2 {
            self.power[i] = Curve4::stream_read(stream, &mut ())?.into_raw();
        }
        for i in 0..2 {
            self.uv[i] = Vec2::stream_read(stream, &mut ())?.into_raw();
        }
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct FillParams {
    basic: BasicParams,
    rgba: [Curve4; 4],
    blend: u32,
    power: Curve4,
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
        for i in 0..4 {
            self.rgba[i] = Curve4::stream_read(stream, &mut ())?.into_raw();
        }
        self.blend = stream.read_u32()?;
        self.power = Curve4::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplLensFlareFlags : u32 {
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

#[repr(C, align(16))]
#[derive(Debug)]
pub struct LensFlareParams {
    basic: BasicParams,
    flags: EplLensFlareFlags,
    templ: u32,
    filter: u32,
    rgba: Curve4,
    brightness: f32,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for LensFlareParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: LensFlareParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl LensFlareParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.flags = EplLensFlareFlags::from_bits_truncate(stream.read_u32()?);
        self.templ = stream.read_u32()?;
        self.filter = stream.read_u32()?;
        self.rgba = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.brightness = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct ColorCorrectionParams {
    basic: BasicParams,
    cyan: f32,
    magenta: f32,
    yellow: f32,
    dodge: f32,
    burn: f32,
    alpha: f32,
    fade: Fade,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for ColorCorrectionParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: ColorCorrectionParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl ColorCorrectionParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.cyan = stream.read_f32()?;
        self.magenta = stream.read_f32()?;
        self.yellow = stream.read_f32()?;
        self.dodge = stream.read_f32()?;
        self.burn = stream.read_f32()?;
        self.alpha = stream.read_f32()?;
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr()]
#[derive(Debug)]
pub struct MonotoneParams {
    basic: BasicParams,
    alpha: f32,
    fade: Fade
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for MonotoneParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: MonotoneParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl MonotoneParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.alpha = stream.read_f32()?;
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[cfg(feature = "v1-core")]
#[repr(C, align(16))]
#[derive(Debug)]
pub struct LensFlareMakeParams {
    basic: BasicParams,
    field8: u32,
    fieldc: u32,
    field10: u32,
    field14: [u8; 60],
    field50: Mat4,
    field90: Curve4
}

#[cfg(all(feature = "serialize", feature = "v1-core"))]
impl<AStream, T> GfdSerialize<AStream, T> for LensFlareMakeParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: LensFlareMakeParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(all(feature = "serialize", feature = "v1-core"))]
impl LensFlareMakeParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.field10 = stream.read_u32()?;
        self.field8 = stream.read_u32()?;
        self.field50 = Mat4::stream_read(stream, &mut ())?.into_raw();
        self.field90 = Curve4::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[cfg(feature = "v2-core")]
#[repr(C, align(16))]
#[derive(Debug)]
pub struct ChromaticAberrationParams {
    basic: BasicParams,
    field10: f32,
    field14: f32,
    field18: f32,
    field1c: Fade,
    field24: Fade,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for ChromaticAberrationParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: ChromaticAberrationParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl ChromaticAberrationParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.field10 = stream.read_f32()?;
        self.field14 = stream.read_f32()?;
        self.field18 = stream.read_f32()?;
        self.field1c = Fade::stream_read(stream, &mut ())?.into_raw();
        self.field24 = Fade::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[cfg(feature = "v1-core")]
#[repr(C, align(16))]
#[derive(Debug)]
pub struct MotionBlurParams {
    basic: BasicParams,
    field8: Curve4,
    field6c: u32,
    field70: Curve4,
    fieldd4: f32,
    fieldd8: f32
}

#[cfg(all(feature = "serialize", feature = "v1-core"))]
impl<AStream, T> GfdSerialize<AStream, T> for MotionBlurParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: MotionBlurParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(all(feature = "serialize", feature = "v1-core"))]
impl MotionBlurParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.field8 = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.field6c = stream.read_u32()?;
        self.field70 = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.fieldd4 = stream.read_f32()?;
        self.fieldd8 = stream.read_f32()?;
        Ok(())
    }
}

#[cfg(feature = "v2-core")]
#[repr(C, align(16))]
#[derive(Debug)]
pub struct ColorCorrectionExcludeToonParams {
    basic: BasicParams,
    field10: f32,
    field14: f32,
    field18: f32,
    field1c: f32,
    field20: f32,
    field24: f32,
    field28: f32,
    field2c: f32,
    field30: f32,
    field34: Fade,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for ColorCorrectionExcludeToonParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: ColorCorrectionExcludeToonParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl ColorCorrectionExcludeToonParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.field10 = stream.read_f32()?;
        self.field14 = stream.read_f32()?;
        self.field18 = stream.read_f32()?;
        self.field1c = stream.read_f32()?;
        self.field20 = stream.read_f32()?;
        self.field24 = stream.read_f32()?;
        self.field28 = stream.read_f32()?;
        self.field2c = stream.read_f32()?;
        self.field30 = stream.read_f32()?;
        self.field34 = Fade::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[cfg(feature = "v1-core")]
#[repr(C, align(16))]
#[derive(Debug)]
pub struct AfterimageBlurParams {
    basic: BasicParams,
    field8: Curve4,
    field6c: u32,
    field70: Curve4,
    fieldd4: f32,
}

#[cfg(all(feature = "serialize", feature = "v1-core"))]
impl<AStream, T> GfdSerialize<AStream, T> for AfterimageBlurParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: AfterimageBlurParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(all(feature = "serialize", feature = "v1-core"))]
impl AfterimageBlurParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.field8 = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.field6c = stream.read_u32()?;
        self.field70 = Curve4::stream_read(stream, &mut ())?.into_raw();
        self.fieldd4 = stream.read_f32()?;
        Ok(())
    }
}