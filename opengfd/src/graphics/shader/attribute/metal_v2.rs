use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    graphics::{
        material::{ 
            Material, 
            MaterialType, 
        },
        shader::{
            flag::{
                Flags1 as ShaderFlag1,
                Flags2 as ShaderFlag2
            },
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags,
};
use glam::{ Vec3, Vec4 };
use crate::kernel::version::GfdVersion;
use crate::utility::misc::{RGBAFloat, RGBFloat};
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MetalFlags: u32 {
        const FLAG0 = 0x00000001;
        const FLAG1 = 0x00000002;
        const FLAG2 = 0x00000004;
        const HasOutline = 0x00000008;
        const FLAG4 = 0x00000010;
        const FLAG5 = 0x00000020;
        const FLAG6 = 0x00000040;
        const FLAG7 = 0x00000080;
        const SubsurfaceScatter = 0x00000100;
        const FLAG9 = 0x00000200;
        const FLAG10 = 0x00000400;
        const FLAG11 = 0x00000800;
        const FLAG12 = 0x00001000;
    }
}

/*
#define FLAG2_SSSS_RECEIVER                      FLAG2_HDR_STAR
#define FLAG2_RAMP_REFLIGHTDIRECTION             FLAG2_HDR_TONEMAP
#define FLAG2_FORCED_BLOOMINTENSITY              FLAG2_SPECULAR_NORMALMAPALPHA
#define FLAG0_MULTI_FOREGROUND                   FLAG0_LIGHT1_POINT
#define FLAG1_SHADOWHATCHING_DISABLE             FLAG1_LIGHTMAP_MODULATE2
#define FLAG0_SHADOWHATCHING_REF_ALPHA_BASECOLOR FLAG0_LIGHT1_DIRECTION
*/

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader File: 45.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct Metal<A = GfdAllocator> 
where A: Allocator + Clone
{
    base_color: RGBAFloat,
    shadow_color: RGBAFloat,
    edge_color: RGBAFloat,
    emissive_color: RGBAFloat,
    specular_color: RGBFloat,
    specular_threshold: f32,
    specular_power: f32,
    p12_19: f32,
    metallic: f32,
    roughness: f32,
    mat_bloom_intensity: f32,
    edge_threshold: f32,
    edge_factor: f32,
    edge_remove_y_axis_factor: f32,
    shadow_threshold: f32,
    shadow_factor: f32,
    p12_7: f32,
    mat_bloom_intensity2: f32,
    p12_8: Vec3,
    p12_11: f32,
    p12_12: f32,
    p12_13: f32,
    field128: f32,
    flags: MetalFlags,
    _allocator: std::marker::PhantomData<A>
}

impl<A> Metal<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Metal<A> 
where A: Allocator + Clone
{
    fn check_billboard_shadow_map(&self) -> bool {
        false
    }
    fn check_inside_14110ba40(&self) -> bool {
        false
    }
    fn check_invisible(&self) -> bool {
        self.base_color.get_alpha_f32() == 0.
    }
    fn check_outline(&self) -> bool {
        self.flags.contains(MetalFlags::HasOutline)
    }
    fn check_render_prio_mod(&self) -> bool {
        false
    }
    fn check_subsurface_scatter(&self) -> bool {
        self.flags.contains(MetalFlags::SubsurfaceScatter)
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
    fn get_tex1_name(&self) -> &'static str { "Base Texture" }
    fn get_tex2_name(&self) -> &'static str { "Normal Texture" }
    fn get_tex3_name(&self) -> &'static str { "Ramp Texture" }
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }
    // R: Metallic, G: Specular, B: Roughness, A: ramp
    fn get_tex6_name(&self) -> &'static str { "Toon Params 2 Texture" }
    fn get_tex9_name(&self) -> &'static str { "Toon Edge Color Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        if self.metallic != 0. {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_REFLECTION;
        }
        if self.edge_threshold != 0.
        && self.edge_color.get_alpha_f32() != 0. {
            *flags |= ShaderFlag2::FLAG2_EDGE_BACKLIGHT;
        }
        if self.flags.contains(MetalFlags::FLAG0) {
            *flags |= ShaderFlag2::FLAG2_TOON_REFERENCE_NORMALMAP;
        }
        if self.flags.contains(MetalFlags::FLAG2) {
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALMAP;
        }
        if self.flags.contains(MetalFlags::FLAG6) {
            *flags |= ShaderFlag2::FLAG2_EDGE_SEMITRANS;
        }
    }
    fn update(&mut self) {
        /* 
        if self.flags.contains(FieldFlags::RemoveDiffuseShadow) {
            // TODO: Remove diffuse shadow
        }
        */
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Metal<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Metal<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Metal<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.edge_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.emissive_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.metallic = stream.read_f32()?;
        self.edge_threshold = stream.read_f32()?;
        self.edge_factor = stream.read_f32()?;
        self.flags = MetalFlags::from_bits_retain(stream.read_u32()?);
        self.p12_7 = stream.read_f32()?;
        self.p12_8 = Vec3::stream_read(stream, &mut ())?.into_raw();
        self.mat_bloom_intensity = stream.read_f32()?;
        self.edge_remove_y_axis_factor = stream.read_f32()?;
        self.p12_11 = stream.has_feature(GfdVersion::MaterialParameterToonAddP17).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.p12_12 = stream.has_feature(GfdVersion::MaterialParameterToonAddP17).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(-1.), |_| Ok(stream.read_f32()?))?;
        self.p12_13 = stream.has_feature(GfdVersion::MaterialParameterToonAddP17).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.mat_bloom_intensity2 = stream.has_feature(GfdVersion::MaterialParameterToonAddP20).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.1), |_| Ok(stream.read_f32()?))?;
        self.specular_color = stream.has_feature(GfdVersion::MaterialParameterMetalAddSpecular).map_or::<Result<RGBFloat, Box<dyn Error>>, _>(Ok(RGBFloat::from_rgb_u32(0x0)), |_| Ok(RGBFloat::stream_read(stream, &mut ())?.into_raw()))?;
        self.specular_threshold = stream.has_feature(GfdVersion::MaterialParameterMetalAddSpecular).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.specular_power = stream.has_feature(GfdVersion::MaterialParameterMetalAddSpecular).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.roughness = stream.has_feature(GfdVersion::MaterialParameterMetalAddSpecular).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.p12_19 = stream.has_feature(GfdVersion::MaterialParameterMetalAddSpecular).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.shadow_color = stream.has_feature(GfdVersion::MaterialParameterMetalAddShadow).map_or::<Result<RGBAFloat, Box<dyn Error>>, _>(Ok(RGBAFloat::from_rgba_u32(0x0)), |_| Ok(RGBAFloat::stream_read(stream, &mut ())?.into_raw()))?;
        self.shadow_threshold = stream.has_feature(GfdVersion::MaterialParameterMetalAddShadow).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.shadow_factor = stream.has_feature(GfdVersion::MaterialParameterMetalAddShadow).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.field128 = 1.;
        Ok(())
    }
}