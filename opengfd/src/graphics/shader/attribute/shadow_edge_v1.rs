use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use bitflags::bitflags;
use allocator_api2::alloc::Allocator;
use crate::graphics::material::{ExtensionObject, ExtensionObjectContext};
use crate::kernel::allocator::GfdAllocator;
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ShadowEdgeFlags : u32 {
        const Flag0 = 1 << 0;
        const DisableEdgeBacklight = 1 << 1;
        const Flag2 = 1 << 2;
        const Flag3 = 1 << 3;
        const Flag4 = 1 << 4;
        const Flag5 = 1 << 5;
        const Flag6 = 1 << 6;
    }
}

#[repr(C)]
pub struct ShadowEdge<A = GfdAllocator>
where A: Allocator + Clone
{
    _super: ExtensionObject<A>,
    light_color: RGBAFloat,
    light_threshold: f32,
    light_factor: f32,
    shadow_color: RGBAFloat,
    shadow_threshold: f32,
    shadow_factor: f32,
    dark_map_speed: f32,
    dark_map_power: f32,
    dark_map_scale: f32,
    dark_map_height: f32,
    dark_map_alpha: f32,
    dark_map_direction: f32,
    dark_grad_height: f32,
    dark_grad_alpha: f32,
    flags: ShadowEdgeFlags,
    alpha: f32,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, ExtensionObjectContext<AObject>> for ShadowEdge<AObject>
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
impl<AObject> ShadowEdge<AObject>
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
        self.dark_map_speed = stream.read_f32()?;
        self.dark_map_power = stream.read_f32()?;
        self.dark_map_scale = stream.read_f32()?;
        self.dark_map_height = stream.read_f32()?;
        self.dark_map_alpha = stream.read_f32()?;
        self.dark_map_direction = stream.read_f32()?;
        self.dark_grad_height = stream.read_f32()?;
        self.dark_grad_alpha = stream.read_f32()?;
        self.flags = ShadowEdgeFlags::from_bits_truncate(stream.read_u32()?);
        self.alpha = 1.;
        Ok(())
    }
}