use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use allocator_api2::alloc::Allocator;
use crate::graphics::material::{ExtensionObject, ExtensionObjectContext};
use crate::kernel::allocator::GfdAllocator;
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, Stream, StreamIODevice};

#[repr(C)]
pub struct Type6<A = GfdAllocator>
where A: Allocator + Clone
{
    _super: ExtensionObject<A>,
    field18: u32,
    field1c: u32,
    field20: u32,
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, ExtensionObjectContext<AObject>> for Type6<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut ExtensionObjectContext<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Type6<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut ExtensionObjectContext<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        self._super = ExtensionObject::<AObject>::new(param.get_id(), param.get_heap_allocator().unwrap());
        self.field18 = stream.read_u32()?;
        self.field1c = stream.read_u32()?;
        self.field20 = stream.read_u32()?;
        Ok(())
    }
}