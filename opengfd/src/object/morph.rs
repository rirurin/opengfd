use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::utility::reference::GfdRcType;
use crate::utility::{
    name::Name,
    reference::Reference
};
use glam::Vec3;
use super::object::Object;
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use bitflags::{bitflags, Flags};
use half::f16;
use opengfd_proc::GfdRcAuto;
use crate::kernel::allocator::GfdAllocator;
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[repr(C)]
#[derive(Debug)]
pub struct MorphController {
    _super: Object,
    num_targets: u32,
    weights: Option<NonNull<f32>>,
    active_morphs: *mut u8,
    name: Name
}

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct MorphTarget<A = GfdAllocator>
where A: Allocator + Clone {
    initial: Option<NonNull<Shape<A>>>,
    targets: Option<NonNull<Shape<A>>>,
    num_targets: i32,
    fvf: MorphTargetAttribute,
    ref_: Reference,
    _allocator: A
}

#[repr(C)]
#[derive(Debug)]
pub struct Shape<A = GfdAllocator>
where A: Allocator + Clone {
    fvf: u32,
    num_vertices: i32,
    // stride: i32,
    vertices: Option<NonNull<Vec3>>,
    _allocator: A
}

bitflags! {
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    pub struct MorphTargetAttribute : u32 {
        const UseSinglePrecision = 1 << 4;
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for MorphTarget<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.ref_ = Reference::new();
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> MorphTarget<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug {
        self.num_targets = stream.read_u32()? as i32;
        self.fvf = MorphTargetAttribute::from_bits_truncate(stream.read_u32()?);
        for _ in 0..self.num_targets {
            let _ = Shape::<AObject>::stream_read(stream, &mut ShapeSerializationContext::new(param.get_heap_allocator().unwrap(), self.fvf));
        }
        Ok(())
    }
}

pub struct ShapeSerializationContext<A>
where A: Allocator + Clone {
    fvf: MorphTargetAttribute,
    _allocator: A
}

impl<A> GfdSerializationUserData<A> for ShapeSerializationContext<A>
where A: Allocator + Clone {
    fn get_heap_allocator(&self) -> Option<A>
    {
        Some(self._allocator.clone())
    }
}

impl<A> ShapeSerializationContext<A>
where A: Allocator + Clone {
    pub(crate) fn new(_allocator: A, fvf: MorphTargetAttribute) -> Self {
        Self { fvf, _allocator}
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, ShapeSerializationContext<AObject>> for Shape<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut ShapeSerializationContext<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Shape<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut ShapeSerializationContext<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug {
        self.fvf = stream.read_u32()?;
        self.num_vertices = stream.read_u32()? as i32;
        for i in 0..self.num_vertices {
            match param.fvf.contains(MorphTargetAttribute::UseSinglePrecision) {
                true => stream.seek(SeekFrom::Current(self.num_vertices as i64 * size_of::<Vec3>() as i64))?,
                false => stream.seek(SeekFrom::Current(self.num_vertices as i64 * size_of::<[f16; 3]>() as i64))?
            };
        }
        Ok(())
    }
}