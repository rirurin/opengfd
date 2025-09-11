use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek, Write};
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    anim::timeline::Timeline,
    effect::parts::Part,
    kernel::allocator::GfdAllocator,
    utility::{
        item_array::ItemArray,
        misc::{ RGBA, Fade, Range },
        name::Name,
        reference::Reference
    }
};
use glam::{Vec3, Vec3A};
use super::{
    node::Node,
    object::Object
};
use std::ptr::NonNull;
use crate::device::ngr::renderer::state::ComparisonFunc;
use crate::graphics::cull::CullObject;
use crate::graphics::curve::CurveType;
use crate::kernel::version::GfdVersion;
use crate::object::object::{CastFromObject, ObjectId};
use crate::utility::name::{NameSerializationContext, NameSerializationHash};
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[derive(Debug)]
pub enum EplError {
    InvalidLeafCategory(u32),
    InvalidParticleEmitterType(u32),
    InvalidCurveType(u16),
    IncorrectCurveType((CurveType, CurveType)),
    InvalidFlashPolygonType(u32),
    InvalidCirclePolygonType(u32),
    InvalidThunderPolygonType(u32),
    InvalidWindPolygonType(u32),
    InvalidModelType(u32),
    InvalidBoardPolygonType(u32),
    InvalidGlitterPolygonType(u32),
    InvalidCameraType(u32),
    InvalidLightType(u32),
    InvalidPostEffectType(u32),
}
impl Error for EplError {}
impl Display for EplError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplFlags : u32 {
        const Flag0 = 1 << 0;
        const Flag1 = 1 << 1;
        const Flag2 = 1 << 2; // Set on load
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
pub struct EPL<A = GfdAllocator> 
where A: Allocator + Clone
{
    super_: Object<A>,
    scale: Vec3A,
    flag: EplFlags,
    dirty: i32,
    time: f32,
    root: Option<NonNull<Node<A>>>,
    timeline: Option<NonNull<Timeline>>,
    leaves: Option<NonNull<ItemArray<NonNull<EPLLeaf<A>>, A>>>,
    rgba: RGBA,
    frequency: f32,
    field60: Vec3,
    field6c: Vec3,
    field78: f32,
    field7c: f32,
    field80: u32,
    ref_: Reference,
    _allocator: A
}

impl<A> CastFromObject for EPL<A>
where A: Allocator + Clone
{
    const OBJECT_ID: ObjectId = ObjectId::EPL;
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for EPL<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        unsafe { this.super_.set_id(Self::OBJECT_ID) };
        this.ref_ = Reference::new();
        this.field60 = Vec3::new(200., 200., 200.);
        this.field6c = Vec3::new(-200., 0., -200.);
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> EPL<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        self.flag = EplFlags::from_bits_truncate(stream.read_u32()? | 4);
        // read hierarchy
        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EplLeafFlags : u32 {
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
pub struct EPLLeaf<A = GfdAllocator>
where A: Allocator + Clone {
    super_: Object<A>,
    scale: Vec3A,
    color: RGBA,
    fade: Fade,
    range: Range,
    rgba: RGBA,
    key_rgba: RGBA,
    field_4c: u8,
    flags: EplLeafFlags,
    name: Name<A>,
    parts: NonNull<Part>,
    _allocator: A
}

impl<A> CastFromObject for EPLLeaf<A>
where A: Allocator + Clone
{
    const OBJECT_ID: ObjectId = ObjectId::EPLLeaf;
}

#[repr(C)]
#[derive(Debug)]
pub struct EPLParts {

}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for EPLLeaf<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        unsafe { this.super_.set_id(Self::OBJECT_ID) };
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> EPLLeaf<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        self.range = stream.has_feature(GfdVersion::EplLeafHasRangeFade).map_or::<Result<Range, Box<dyn Error>>, _>(Ok(Range::default()), |_| Ok(Range::stream_read(stream, &mut ())?.into_raw()))?;
        self.fade = stream.has_feature(GfdVersion::EplLeafHasRangeFade).map_or::<Result<Fade, Box<dyn Error>>, _>(Ok(Fade::default()), |_| Ok(Fade::stream_read(stream, &mut ())?.into_raw()))?;
        self.flags = EplLeafFlags::from_bits_truncate(stream.read_u32()? | 4);
        if stream.get_header_version() < GfdVersion::EplLeafFlag3004 as u32 {
            self.flags |= EplLeafFlags::Flag13 | EplLeafFlags::Flag12 | EplLeafFlags::Flag2;
        }
        self.name = Name::<AObject>::stream_read(stream, &mut NameSerializationContext::new(param.get_heap_allocator().unwrap(), NameSerializationHash))?.into_raw();
        Ok(())
    }
}