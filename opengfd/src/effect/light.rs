use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use crate::{
    anim::key::KeyController,
    effect::{
        misc::Adjustment,
        resources::{ EPLParameter, EPLMaterial }
    },
    object::node::Node,
    utility::misc::{ Fade, RGBA }
};
use glam::{Vec3A, Mat4, Vec3};
use crate::kernel::allocator::GfdAllocator;
use crate::object::epl::EplError;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};
use bitflags::bitflags;
use crate::graphics::curve::Curve4;
use crate::kernel::version::GfdVersion;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LightType
{
    Mesh = 1,
    Scene,
    Point,
    Spot
}

impl TryFrom<u32> for LightType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Mesh),
            2 => Ok(Self::Scene),
            3 => Ok(Self::Point),
            4 => Ok(Self::Spot),
            v => Err(EplError::InvalidLightType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Light<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: LightType,
    color: RGBA, 
    dirty: u32,
    current: KeyController,
    next: KeyController,
    material: Option<NonNull<EPLMaterial<A>>>,
    node: Option<NonNull<Node<A>>>,
    parts: *mut u8,
    params: NonNull<EPLParameter<BasicParams, A>>,
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Light<AObject>
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
impl<AObject> Light<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            LightType::Mesh => EPLParameter::<MeshParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            LightType::Scene => EPLParameter::<SceneParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            LightType::Point => EPLParameter::<PointParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            LightType::Spot => EPLParameter::<SpotParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
    pub struct EplLightFlags : u32 {
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
    flag: EplLightFlags,
    adjust: Adjustment,
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
        self.flag = EplLightFlags::from_bits_truncate(stream.read_u32()?);
        self.adjust = Adjustment::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshParams {
    basic: BasicParams,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for MeshParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: MeshParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl MeshParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SceneParams {
    basic: BasicParams,
    life: f32,
    fade: Fade,
    ambient: RGBA,
    diffuse: RGBA,
    specular: RGBA,
    rotate: Vec3,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SceneParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SceneParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SceneParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.life = stream.read_f32()?;
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        self.ambient = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.diffuse = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.specular = RGBA::stream_read(stream, &mut ())?.into_raw();
        self.rotate = stream
            .has_feature(GfdVersion::EplLightSceneHasRotate)
            .map_or::<Result<Vec3, Box<dyn Error>>, _>(
                Ok(Vec3::default()),
                |_| Ok(Vec3::stream_read(stream, &mut ())?.into_raw())
            )?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PointParams {
    basic: BasicParams,
    life: f32,
    fade: Fade,
    field18: Curve4,
    field7c: [f32; 3],
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for PointParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: PointParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl PointParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.life = stream.read_f32()?;
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        self.field18 = Curve4::stream_read(stream, &mut ())?.into_raw();
        for i in 0..3 {
            self.field7c[i] = stream.read_f32()?;
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SpotParams {
    basic: BasicParams,
    life: f32,
    fade: Fade,
    field18: Curve4,
    field7c: [f32; 5],
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for SpotParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: SpotParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl SpotParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.life = stream.read_f32()?;
        self.fade = Fade::stream_read(stream, &mut ())?.into_raw();
        self.field18 = Curve4::stream_read(stream, &mut ())?.into_raw();
        for i in 0..5 {
            self.field7c[i] = stream.read_f32()?;
        }
        Ok(())
    }
}