use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::graphics::material::{ExtensionObject, ExtensionObjectContext};
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ToonFlags : u32 {
        const LightNormalMap = 1 << 0;
        const LightAdd = 1 << 1;
        const ShadowNormalMap = 1 << 2;
        const LockYAxis = 1 << 3;
        const LightNormalMapAlphaMask = 1 << 4;
        const LightDiffuseMapAlphaMask = 1 << 5;
        const LightShadowMapAlphaMask = 1 << 6;
    }
}

#[repr(C)]
pub struct Toon<A = GfdAllocator>
where A: Allocator + Clone {
    _super: ExtensionObject<A>,
    light_color: RGBAFloat,
    light_threshold: f32,
    light_factor: f32,
    shadow_brightness: f32,
    shadow_threshold: f32,
    shadow_factor: f32,
    flags: ToonFlags,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, ExtensionObjectContext<AObject>> for Toon<AObject>
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
impl<AObject> Toon<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut ExtensionObjectContext<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self._super = ExtensionObject::<AObject>::new(param.get_id(), param.get_heap_allocator().unwrap());
        if stream.get_header_version() >= GfdVersion::MaterialExtensionToonV3 as u32 {
            self.light_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
            self.light_threshold = stream.read_f32()?;
            self.light_factor = stream.read_f32()?;
            self.shadow_brightness = stream.read_f32()?;
            self.shadow_threshold = stream.read_f32()?;
            self.shadow_factor = stream.read_f32()?;
            self.flags = ToonFlags::from_bits_truncate(stream.read_u32()?);
        } else if stream.get_header_version() >= GfdVersion::MaterialExtensionToonV2 as u32 {
            self.light_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
            self.light_threshold = stream.read_f32()?;
            self.light_factor = stream.read_f32()?;
            self.shadow_brightness = stream.read_f32()?;
            self.shadow_threshold = stream.read_f32()?;
            self.shadow_factor = stream.read_f32()?;
            if stream.read_u8()? != 0 {
                self.flags |= ToonFlags::LightNormalMap;
            }
            if stream.read_u8()? != 0 {
                self.flags |= ToonFlags::LightAdd;
            }
            if stream.read_u8()? != 0 {
                self.flags |= ToonFlags::ShadowNormalMap;
            }
            if stream.get_header_version() >= GfdVersion::MaterialExtensionToonV2LockYAxis as u32
            && stream.read_u8()? != 0 {
                self.flags |= ToonFlags::LockYAxis;
            }
        } else {
            self.light_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
            self.light_threshold = stream.read_f32()?;
            self.light_factor = stream.read_f32()?;
            self.shadow_brightness = 0.25;
            self.shadow_threshold = stream.read_f32()?;
            self.shadow_factor = stream.read_f32()?;
        }
        Ok(())
    }
}