use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use allocator_api2::alloc::Allocator;
use crate::graphics::material::{ExtensionObject, ExtensionObjectContext};
use crate::kernel::allocator::GfdAllocator;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, Stream, StreamIODevice};

#[repr(C)]
pub struct Type5<A = GfdAllocator>
where A: Allocator + Clone
{
    _super: ExtensionObject<A>,
    field18: u32,
    field1c: u32,
    field20: f32,
    field24: f32,
    field28: RGBAFloat,
    field38: f32,
    field3c: f32,
    field40: f32,
    field44: f32,
    field48: f32,
    field4c: f32,
    field50: f32,
    field54: RGBAFloat
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, ExtensionObjectContext<AObject>> for Type5<AObject>
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
impl<AObject> Type5<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut ExtensionObjectContext<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self._super = ExtensionObject::<AObject>::new(param.get_id(), param.get_heap_allocator().unwrap());
        self.field18 = stream.read_u32()?;
        self.field1c = stream.read_u32()?;
        self.field20 = stream.read_f32()?;
        self.field24 = stream.read_f32()?;
        self.field28 = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.field38 = stream.read_f32()?;
        self.field3c = stream.read_f32()?;
        self.field40 = stream.read_f32()?;
        self.field44 = stream.read_f32()?;
        self.field48 = stream.read_f32()?;
        self.field4c = stream.read_f32()?;
        self.field50 = stream.read_f32()?;
        self.field54 = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}