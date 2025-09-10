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
    pub struct EdgeFlags : u32 {
        const NormalMap = 1 << 0;
        const Backlight = 1 << 1;
        const LightAdd = 1 << 2;
        const CavernMap = 1 << 3;
        const LightNormalMapAlphaMask = 1 << 4;
        const LockYAxis = 1 << 5;
        const LightDiffuseMapAlphaMask = 1 << 6;
        const LightShadowMapAlphaMask = 1 << 7;
    }
}

#[repr(C)]
pub struct Edge<A = GfdAllocator>
where A: Allocator + Clone
{
    _super: ExtensionObject<A>,
    light_color: RGBAFloat,
    light_threshold: f32,
    light_factor: f32,
    shadow_color: RGBAFloat,
    shadow_threshold: f32,
    shadow_factor: f32,
    flags: EdgeFlags,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, ExtensionObjectContext<AObject>> for Edge<AObject>
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
impl<AObject> Edge<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut ExtensionObjectContext<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self._super = ExtensionObject::<AObject>::new(param.get_id(), param.get_heap_allocator().unwrap());
        self.light_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.light_threshold = stream.read_f32()?;
        self.light_factor = stream.read_f32()?;
        self.shadow_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.shadow_threshold = stream.read_f32()?;
        self.shadow_factor = stream.read_f32()?;
        if stream.get_header_version() >= GfdVersion::MaterialExtensionToonV3 as u32 {
            self.flags = EdgeFlags::from_bits_truncate(stream.read_u32()?);
        } else {
            if stream.read_u8()? != 0 {
                self.flags |= EdgeFlags::NormalMap;
            }
            if stream.get_header_version() >= GfdVersion::MaterialExtensionEdgeBacklight as u32 && stream.read_u8()? != 0 {
                self.flags |= EdgeFlags::Backlight;
            }
            if stream.get_header_version() >= GfdVersion::MaterialExtensionEdgeLightAdd as u32 && stream.read_u8()? != 0 {
                self.flags |= EdgeFlags::LightAdd;
            }
            if stream.get_header_version() >= GfdVersion::MaterialExtensionEdgeCavernmap as u32 && stream.read_u8()? != 0 {
                self.flags |= EdgeFlags::CavernMap;
            }
        }
        Ok(())
    }
}