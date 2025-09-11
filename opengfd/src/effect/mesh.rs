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
    utility::misc::RGBA
};
use glam::{ Vec3A, Mat4 };
use crate::kernel::allocator::GfdAllocator;
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};
use bitflags::bitflags;
use crate::kernel::version::GfdVersion;
use crate::object::epl::EplError;
use crate::utility::misc::Range;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshType
{
    ThreeD = 1,
    TwoD
}

impl TryFrom<u32> for MeshType {
    type Error = EplError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ThreeD),
            2 => Ok(Self::TwoD),
            v => Err(EplError::InvalidModelType(v))
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Model<A = GfdAllocator>
where A: Allocator + Clone {
    transform: Mat4,
    scale: Vec3A,
    type_: MeshType,
    color: RGBA, 
    dirty: u32,
    current: KeyController,
    next: KeyController,
    material: Option<NonNull<EPLMaterial<A>>>,
    node: Option<NonNull<Node<A>>>,
    parts: Option<NonNull<u8>>,
    params: NonNull<EPLParameter<BasicParams, A>>,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Model<AObject>
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
impl<AObject> Model<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.type_ = stream.read_u32()?.try_into()?;
        self.params = unsafe { NonNull::new_unchecked(match self.type_ {
            MeshType::ThreeD => EPLParameter::<ThreeDParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
            MeshType::TwoD => EPLParameter::<TwoDParams, AObject>::stream_read(stream, param)?.into_raw().as_ptr() as *mut EPLParameter<BasicParams, AObject>,
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
    pub struct EplModelFlags : u32 {
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

#[cfg(feature = "v2-core")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flag: EplModelFlags,
    adjust: Adjustment,
    fieldc: Range,
    field14: Range,
    field1c: f32,
    field20: u32,
    field24: f32,
    field28: u32,
}

#[cfg(feature = "v1-core")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicParams {
    flag: EplModelFlags,
    adjust: Adjustment,
    extra_scroll_texture_opacity: f32,
    field10: f32,
    field14: f32,
    field18: f32,
    field1c: f32,
    field20: f32,
    field24: u32,
    extra_scroll_texture_brightness1: f32,
    extra_scroll_texture_brightness2: f32,
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
        self.flag = EplModelFlags::from_bits_truncate(stream.read_u32()?);
        self.adjust = stream
            .has_feature(GfdVersion::EplBehaviorAdjustmentParams)
            .map_or::<Result<Adjustment, Box<dyn Error>>, _>(
                Ok(Adjustment::default()),
                |_| Ok(Adjustment::stream_read(stream, &mut ())?.into_raw())
            )?;
        #[cfg(feature = "v2-core")]
        {
            self.fieldc = stream
                .has_feature(GfdVersion::EplModelExtraFields)
                .map_or::<Result<Range, Box<dyn Error>>, _>(
                    Ok(Range::default()),
                    |_| Ok(Range::stream_read(stream, &mut ())?.into_raw())
                )?;
            self.field14 = stream
                .has_feature(GfdVersion::EplModelExtraFields)
                .map_or::<Result<Range, Box<dyn Error>>, _>(
                    Ok(Range::default()),
                    |_| Ok(Range::stream_read(stream, &mut ())?.into_raw())
                )?;
            self.field1c = stream
                .has_feature(GfdVersion::EplModelExtraFields)
                .map_or::<Result<f32, Box<dyn Error>>, _>(
                    Ok(0.),
                    |_| Ok(stream.read_f32()?)
                )?;
            self.field20 = stream
                .has_feature(GfdVersion::EplModelExtraFields)
                .map_or::<Result<u32, Box<dyn Error>>, _>(
                    Ok(0),
                    |_| Ok(stream.read_u32()?)
                )?;
            self.field24 = stream
                .has_feature(GfdVersion::EplModelExtraFields)
                .map_or::<Result<f32, Box<dyn Error>>, _>(
                    Ok(0.),
                    |_| Ok(stream.read_f32()?)
                )?;
            self.field28 = stream
                .has_feature(GfdVersion::EplModelExtraFields)
                .map_or::<Result<u32, Box<dyn Error>>, _>(
                    Ok(0x19771022),
                    |_| Ok(stream.read_u32()?)
                )?;
        }
        #[cfg(feature = "v1-core")]
        {
            if self.flags.contains(EplModelFlags::Flag28) {
                self.extra_scroll_texture_opacity = stream.read_f32()?;
                self.field10 = stream.read_f32()?;
                self.field14 = stream.read_f32()?;
                self.field18 = stream.read_f32()?;
                self.field1c = stream.read_f32()?;
                self.field20 = stream.read_f32()?;
                self.field24 = stream.read_u32()?;
                self.extra_scroll_texture_brightness1 = stream.read_f32()?;
                self.extra_scroll_texture_brightness2 = stream.read_f32()?;
            }
        }
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ThreeDParams {
    basic: BasicParams,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for ThreeDParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: ThreeDParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl ThreeDParams {
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
pub struct TwoDParams {
    basic: BasicParams,
    distance: f32
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for TwoDParams
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: TwoDParams = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl TwoDParams {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug,
    {
        self.basic = BasicParams::stream_read(stream, &mut ())?.into_raw();
        self.distance = stream.read_f32()?;
        Ok(())
    }
}