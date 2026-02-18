use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    graphics::{
        material::{ Material, MaterialType },
        shader::{
            flag::{
                Flags0 as ShaderFlag0,
                Flags2 as ShaderFlag2
            },
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags,
};
use crate::graphics::material::params::MaterialId;
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct WaterFlags: u32 {
        const InfluencedBySky = 0x00000001;
        const HasWaterReflection = 0x00000002;
        const IsInfinite = 0x00000004;
        const OutlineAttenuationInvalid = 0x00000008;
    }
}

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader file: 23.HLSL

#[repr(C)]
#[derive(Debug)]
pub struct Water<A = GfdAllocator> 
where A: Allocator + Clone
{
    p5_0: f32,
    p5_1: f32,
    tc_scale: f32,
    p5_3: f32,
    ocean_depth_scale: f32,
    disturbance_camera_scale: f32,
    disturbance_depth_scale: f32,
    scattering_camera_scale: f32,
    disturbance_tolerance: f32,
    foam_distance: f32,
    caustics_tolerance: f32,
    p5_11: f32,
    texture_animation_speed: f32,
    p5_13: f32,
    flags: WaterFlags,
    _allocator: A
}

impl<A> Water<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Water<A> 
where A: Allocator + Clone
{
    fn check_billboard_shadow_map(&self) -> bool {
        false
    }
    fn check_inside_14110ba40(&self) -> bool {
        false
    }
    fn check_invisible(&self) -> bool {
        false
    }
    fn check_outline(&self) -> bool {
        false
    }
    fn check_render_prio_mod(&self) -> bool {
        false
    }
    fn check_subsurface_scatter(&self) -> bool {
        false
    }
    fn check_toon_flag_8000(&self) -> bool {
        false
    }
    fn check_translucency(&self) -> bool {
        false
    }
    fn check_transparent_14107980(&self) -> bool {
        false
    }
    fn get_base_color_opacity(&self) -> f32 {
        0.
    }
    fn get_shadow_link_func(&self) -> u8 {
        0
    }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        if self.flags.contains(WaterFlags::InfluencedBySky) {
            *flags |= ShaderFlag2::FLAG2_SKY;
        }
        if self.flags.contains(WaterFlags::HasWaterReflection) {
            *flags |= ShaderFlag2::FLAG2_WATER_REFLECTION;
        }
        if self.flags.contains(WaterFlags::IsInfinite) {
            *flags |= ShaderFlag0::FLAG0_INFINITE;
        }
        if self.flags.contains(WaterFlags::OutlineAttenuationInvalid) {
            *flags |= ShaderFlag0::FLAG0_OUTLINE_ATTENUATION_INVALID;
        }
    }
    fn update(&mut self) {
        /* 
        if self.flags.contains(FieldFlags::RemoveDiffuseShadow) {
            // TODO: Remove diffuse shadow
        }
        */
    }
    fn get_material_id(&self) -> MaterialId {
        MaterialId::Water
    }
    fn get_shader_id(&self) -> u32 {
        0x93
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Water<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Water<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Water<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.p5_0 = stream.read_f32()?;
        self.p5_1 = stream.read_f32()?;
        self.tc_scale = stream.read_f32()?;
        self.p5_3 = stream.read_f32()?;
        self.ocean_depth_scale = stream.read_f32()?;
        self.disturbance_camera_scale = stream.read_f32()?;
        self.disturbance_depth_scale = stream.read_f32()?;
        self.scattering_camera_scale = stream.read_f32()?;
        self.disturbance_tolerance = stream.read_f32()?;
        self.foam_distance = stream.read_f32()?;
        self.caustics_tolerance = stream.read_f32()?;
        self.p5_11 = stream.has_feature(GfdVersion::MaterialParameterWaterAddTextureSpeed).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.texture_animation_speed = stream.has_feature(GfdVersion::MaterialParameterWaterAddTextureSpeed).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.p5_13 = stream.has_feature(GfdVersion::EnvAddInfiniteOcean_LUTRecolorParams).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.flags = stream.has_feature(GfdVersion::MaterialParameterWaterAddFlags).map_or::<Result<WaterFlags, Box<dyn Error>>, _>(Ok(WaterFlags::empty()), |_| Ok(WaterFlags::from_bits_truncate(stream.read_u32()?)))?;
        Ok(())
    }
}