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
                Flags0 as ShaderFlag0,
                Flags1 as ShaderFlag1,
                Flags2 as ShaderFlag2
            },
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags,
};
use glam::Vec4;
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DistortionFlags: u32 {
        const FlowMapMultiAsMask = 0x00000001;
        const HDRStar = 0x00000002;
        const FlowMapRimTransNeg = 0x00000004;
        const FlowMapBackgroundDistort = 0x00000008;
        const FlowMapMultiAlphaColorBlendOnly = 0x00000010;
        const FlowMapAlphaDistortion = 0x00000020;
        const FlowMapAlphaMaskDistortion = 0x00000040;
        const FlowMapApplyAlphaOnly = 0x00000080;
        const SoftParticle = 0x00000100;
        const ForcedBloomIntensity = 0x00000200;
        const Fitting = 0x00000400;
        const FlowmapColorCorrectDisable = 1 << 0xb;
        const MultiFitting = 1 << 0xc;
        const FlowMapMultiRefAlphaBaseColor = 1 << 0xd;
        const FlowMapBloomRefAlphaMultiColor = 1 << 0xe;
        const FLAG15 = 1 << 0xf;
    }
}

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader File: 21.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct CharacterDistortion<A = GfdAllocator> 
where A: Allocator + Clone
{
    base_color: RGBAFloat,
    emissive_color: RGBAFloat,
    distortion_power: f32,
    distortion_threshold: f32,
    p4_4: f32,
    bloom_strength: f32,
    fitting_tile: f32,
    multi_fitting_tile: f32,
    fieldc8: f32,
    flags: DistortionFlags,
    _allocator: std::marker::PhantomData<A>
}

impl<A> CharacterDistortion<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for CharacterDistortion<A> 
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
        if self.flags.contains(DistortionFlags::FLAG15) {
            return false;
        }
        !(self.base_color.get_alpha_f32() >= 1. && self.get_material().get_constant() as i8 == -1)
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
    fn get_tex3_name(&self) -> &'static str { "Dissolve Texture" }
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }
    fn get_tex6_name(&self) -> &'static str { "Emissive Texture" }
    fn get_tex7_name(&self) -> &'static str { "Transparency Texture" }
    fn get_tex8_name(&self) -> &'static str { "Distortion Texture" }
    fn get_tex9_name(&self) -> &'static str { "Alpha Mask Texture" }
    fn get_tex10_name(&self) -> &'static str { "Alpha Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        if self.flags.contains(DistortionFlags::FlowMapMultiAsMask) {
            // #define FLAG2_FLOWMAP_MULTIASMASK                FLAG2_HDR_TONEMAP
            *flags |= ShaderFlag2::FLAG2_HDR_TONEMAP;
        }
        if self.flags.contains(DistortionFlags::HDRStar) {
            //  #define FLAG2_FLOWMAP_RIMTRANSPARENCY            FLAG2_HDR_STAR
            *flags |= ShaderFlag2::FLAG2_HDR_STAR;
            if self.flags.contains(DistortionFlags::FlowMapRimTransNeg) {
                // #define FLAG2_FLOWMAP_RIMTRANSNEG                FLAG2_EDGE_CAVERNMAP
                *flags |= ShaderFlag2::FLAG2_EDGE_CAVERNMAP;
            }
        }
        if self.flags.contains(DistortionFlags::FlowMapBackgroundDistort) {
            // #define FLAG2_FLOWMAP_BGDISTORTION               FLAG2_EDGE_REFERENCE_NORMALALPHA
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALALPHA;
        }
        if self.flags.contains(DistortionFlags::FlowMapMultiAlphaColorBlendOnly) {
            // #define FLAG2_FLOWMAP_MULTIALPHA_COLORBLENDONLY  FLAG2_EDGE_REFERENCE_DIFFUSEALPHA
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_DIFFUSEALPHA;
        }
        if self.flags.contains(DistortionFlags::FlowMapAlphaDistortion) {
            // #define FLAG2_FLOWMAP_ALPHADISTORTION            FLAG2_TOON_REFERENCE_NORMALMAP
            *flags |= ShaderFlag2::FLAG2_TOON_REFERENCE_NORMALMAP;
        }
        if self.flags.contains(DistortionFlags::FlowMapAlphaDistortion) {
            // #define FLAG2_FLOWMAP_ALPHAMASKDISTORTION        FLAG2_TOON_REMOVAL_LIGHT_YAXIS
            *flags |= ShaderFlag2::FLAG2_TOON_REMOVAL_LIGHT_YAXIS;
        }
        if self.flags.contains(DistortionFlags::FlowMapApplyAlphaOnly) {
            // #define FLAG2_FLOWMAP_APPLYALPHAONLY             FLAG2_EDGE_BACKLIGHT
            *flags |= ShaderFlag2::FLAG2_EDGE_BACKLIGHT;
        }
        if self.flags.contains(DistortionFlags::SoftParticle) {
            *flags |= ShaderFlag2::FLAG2_SOFT_PARTICLE;
        }
        if self.flags.contains(DistortionFlags::ForcedBloomIntensity) {
            // #define FLAG2_FORCED_BLOOMINTENSITY              FLAG2_SPECULAR_NORMALMAPALPHA
            *flags |= ShaderFlag2::FLAG2_SPECULAR_NORMALMAPALPHA;
        }
        if self.flags.contains(DistortionFlags::Fitting) {
            // #define FLAG0_FITTING                            FLAG0_LIGHT1_SPOT
            *flags |= ShaderFlag0::FLAG0_LIGHT1_SPOT;
        }
        if self.flags.contains(DistortionFlags::FlowmapColorCorrectDisable) {
            // #define FLAG0_FLOWMAP_COLORCORRECT_DISABLE       FLAG0_LIGHT2_DIRECTION
            *flags |= ShaderFlag0::FLAG0_LIGHT2_DIRECTION;
        }
        if self.flags.contains(DistortionFlags::MultiFitting) {
            // #define FLAG0_MULTI_FITTING                      FLAG0_LIGHT0_SPOT
            *flags |= ShaderFlag0::FLAG0_LIGHT0_SPOT;
        }
        if self.flags.contains(DistortionFlags::FlowMapMultiRefAlphaBaseColor) {
            // #define FLAG1_FLOWMAP_MULTI_REF_ALPHA_BASECOLOR  FLAG1_LIGHTMAP_MODULATE2
            *flags |= ShaderFlag1::FLAG1_LIGHTMAP_MODULATE2;
        }
        if self.flags.contains(DistortionFlags::FlowMapBloomRefAlphaMultiColor) {
            // #define FLAG0_FLOWMAP_BLOOM_REF_ALPHA_MULTICOLOR FLAG0_LIGHT2_SPOT
            *flags |= ShaderFlag0::FLAG0_LIGHT2_SPOT;
        }
    }
    fn update(&mut self) {
        /* 
        if self.flags.contains(FieldFlags::RemoveDiffuseShadow) {
            // TODO: Remove diffuse shadow
        }
        */
    }
    fn get_shader_id(&self) -> u32 {
        0x92
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for CharacterDistortion<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: CharacterDistortion<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> CharacterDistortion<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.emissive_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.distortion_power = stream.read_f32()?;
        self.distortion_threshold = stream.read_f32()?;
        self.p4_4 = stream.read_f32()?;
        self.flags = DistortionFlags::from_bits_truncate(stream.read_u32()?);
        self.bloom_strength = stream.has_feature(GfdVersion::MaterialParameter4AddBloomIntensity).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.5), |_| Ok(stream.read_f32()?))?;
        self.fitting_tile = stream.has_feature(GfdVersion::MaterialParameterDistortAddMultiFittingTile).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.multi_fitting_tile = stream.has_feature(GfdVersion::MaterialParameterDistortAddP8).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.fieldc8 = 1.;
        Ok(())
    }
}