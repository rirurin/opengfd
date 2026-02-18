use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::marker::PhantomData;
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
use glam::{Vec3, Vec3A, Vec4};
use crate::graphics::material::{BlendType, MaterialFlags2};
use crate::graphics::material::params::MaterialId;
use crate::kernel::version::GfdVersion;
use crate::utility::misc::{RGBAFloat, RGBFloat};
#[cfg(feature = "serialize")]
use crate::utility::stream::{DeserializationStrategy, DeserializationStack, GfdSerialize, Stream, StreamIODevice};
// From https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord )]
    pub struct ToonBaseFlags : u32 {
        const Flag0 = 0x00000001;
        const Flag1 = 0x00000002;
        const Flag2 = 0x00000004;
        const Flag3 = 0x00000008;
        const Flag4 = 0x00000010;
        const Flag5 = 0x00000020;
        const Flag6 = 0x00000040;
        const Flag7 = 0x00000080;
        const Flag8 = 0x00000100;
        const Flag9 = 0x00000200;
        const Flag10 = 0x00000400;
        const Flag11 = 0x00000800;
        const Flag12 = 0x00001000;
        const Flag13 = 0x00002000;
        const Flag14 = 0x00004000;
        const Flag15 = 0x00008000;
        const Flag16 = 0x00010000;
        const Flag17 = 0x00020000;
        const Flag18 = 0x00040000;
        const Flag19 = 0x00080000;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Toon {
    pub(super) base_color: RGBAFloat,
    pub(super) shadow_color: RGBAFloat,
    pub(super) edge_color: RGBAFloat,
    pub(super) emissive_color: RGBAFloat,
    pub(super) specular_color: RGBFloat,
    pub(super) specular_threshold: f32,
    pub(super) specular_power: f32,
    pub(super) metallic: f32,
    pub(super) roughness: f32,
    pub(super) bloom_strength: f32,
    pub(super) edge_threshold: f32,
    pub(super) edge_factor: f32,
    pub(super) edge_remove_y_axis_factor: f32,
    pub(super) shadow_threshold: f32,
    pub(super) shadow_factor: f32,
    pub(super) field74: f32,
    pub(super) field78: f32,
    pub(super) field7c: Vec3,
    pub(super) field88: f32,
    pub(super) field8c: f32,
    pub(super) field90: f32,
    pub(super) field94: f32,
    pub(super) fitting_tile: f32,
    pub(super) multi_fitting_tile: f32,
    pub(super) flags: ToonBaseFlags
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord )]
    pub struct CharaToonFlags : u32 {
        const ToonRefNormalMap = 0x00000001;
        const ToonRemoveLightYAxis = 0x00000002;
        const EdgeRefNormalMap = 0x00000004;
        const Flag3 = 0x00000008;
        const Flag4 = 0x00000010;
        const EdgeRefNormalAlpha = 0x00000020;
        const EdgeSemitrans = 0x00000040;
        const EdgeRemoveLightYAxis = 0x00000080;
        const Flag8 = 0x00000100;
        const SubsurfaceScatterReceiver = 0x00000200;
        const Punchthrough = 0x00000400;
        const Flag11 = 0x00000800;
        const Flag12 = 0x00001000;
        const ApplyPBRLight = 0x00002000;
        const ForcedBloomIntensity = 0x00004000;
        const Flag15 = 0x00008000;
        const RefShadowColorAlpha = 0x00010000;
        const MultiLayerBaseTexture = 0x00020000;
        const MultiForeground = 0x00040000;
        const Fitting= 0x00080000;
        const MultiRefBaseAlpha = 1 << 20;
        const MultiFitting = 1 << 21;
        const ReflectionBackground = 1 << 22;
        const ShadowHatchingDisable = 1 << 23;
        const ShadowHatchingRefAlphaBaseColor = 1 << 24;
    }
}


/// Source File: 11.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct CharacterToon<A = GfdAllocator>
where A: Allocator + Clone
{
    _impl: Toon,
    _alloc: std::marker::PhantomData<A>
}

impl<A> CharacterToon<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }

    pub fn has_flag(&self, flag: CharaToonFlags) -> bool {
        self._impl.flags.contains(ToonBaseFlags::from_bits_truncate(flag.bits()))
    }
}

impl<A> MaterialType for CharacterToon<A> 
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
        if self.has_flag(CharaToonFlags::Punchthrough) {
            return false;
        }
        !(self._impl.base_color.get_alpha_f32() >= 1. && self.get_material().get_constant() as i8 == -1)
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
    fn get_tex3_name(&self) -> &'static str { "Toon Shadow Color Texture" }
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }
    // R: Metallic, G: Specular, B: Emissive Mask, A: Reserve
    fn get_tex6_name(&self) -> &'static str { "Toon Params 2 Texture" }
    // R: Shadow Mask, G: Edge Mask, B: Shadow Fix, A: Bloom Intensity
    fn get_tex8_name(&self) -> &'static str { "Toon Params Texture" }
    fn get_tex9_name(&self) -> &'static str { "Toon Edge Color Texture" }

    fn set_shader_flags(&self, vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        if self._impl.metallic > 0. {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_REFLECTION;
        }
        if self._impl.edge_threshold > 0.
        && self._impl.edge_color.get_alpha_f32() > 0. {
            *flags |= ShaderFlag2::FLAG2_EDGE_BACKLIGHT;
        }
        if self.has_flag(CharaToonFlags::ToonRefNormalMap) {
            *flags |= ShaderFlag2::FLAG2_TOON_REFERENCE_NORMALMAP;
        }
        if self.has_flag(CharaToonFlags::ToonRemoveLightYAxis) {
            *flags |= ShaderFlag2::FLAG2_TOON_REMOVAL_LIGHT_YAXIS;
        }
        if self.has_flag(CharaToonFlags::EdgeRefNormalMap) {
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALMAP;
        }
        if self.has_flag(CharaToonFlags::EdgeRefNormalAlpha) {
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALALPHA;
        }
        if self.has_flag(CharaToonFlags::EdgeSemitrans) {
            *flags |= ShaderFlag2::FLAG2_EDGE_SEMITRANS;
        }
        if self.has_flag(CharaToonFlags::EdgeRemoveLightYAxis) {
            *flags |= ShaderFlag0::FLAG0_EDGE_REMOVAL_LIGHT_YAXIS;
        }
        if self._impl.specular_power > 0.
            && self._impl.specular_threshold > 0.
            && self._impl.specular_color != RGBFloat::default() {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_SPECULAR;
        }
        if vtx.contains(VertexAttributeFlags::Color2) {
            *flags |= ShaderFlag0::FLAG0_OUTLINE;
        }
        if self.has_flag(CharaToonFlags::Punchthrough)
            && self.get_material().get_blend().get_type() == BlendType::Opaque
            && (
                self._impl.base_color.get_alpha_f32() < 1.
                || self.get_material().get_flag2().contains(MaterialFlags2::Punchthrough)
                || self.get_material().get_constant() as i8 != -1
            )
        {
            *flags |= ShaderFlag2::FLAG2_PUNCHTHROUGH;
        }
        if self.has_flag(CharaToonFlags::SubsurfaceScatterReceiver) {
            // #define FLAG2_SSSS_RECEIVER                      FLAG2_HDR_STAR
            *flags |= ShaderFlag2::FLAG2_HDR_STAR;
        }
        if self.has_flag(CharaToonFlags::ApplyPBRLight) {
            *flags |= ShaderFlag2::FLAG2_APPLY_PBR_LIGHT;
        }
        if self.has_flag(CharaToonFlags::ForcedBloomIntensity) {
            // #define FLAG2_FORCED_BLOOMINTENSITY              FLAG2_SPECULAR_NORMALMAPALPHA
            *flags |= ShaderFlag2::FLAG2_SPECULAR_NORMALMAPALPHA;
        }
        if self.has_flag(CharaToonFlags::RefShadowColorAlpha) {
            // #define FLAG0_REF_SHADOWCOLOR_ALPHA              FLAG0_LIGHT2_POINT
            *flags |= ShaderFlag0::FLAG0_LIGHT2_POINT;
        }
        if self.has_flag(CharaToonFlags::MultiLayerBaseTexture) {
            // #define FLAG0_MULTI_LAYER_BASETEXTURE            FLAG0_LIGHT2_SPOT
            *flags |= ShaderFlag0::FLAG0_LIGHT2_SPOT;
        }
        if self.has_flag(CharaToonFlags::MultiForeground) {
            // #define FLAG0_MULTI_FOREGROUND                   FLAG0_LIGHT1_POINT
            *flags |= ShaderFlag0::FLAG0_LIGHT1_POINT;
        }
        if self.has_flag(CharaToonFlags::Fitting) {
            // #define FLAG0_FITTING                            FLAG0_LIGHT1_SPOT
            *flags |= ShaderFlag0::FLAG0_LIGHT1_SPOT;
        }
        if self.has_flag(CharaToonFlags::MultiRefBaseAlpha) {
            // #define FLAG0_MULTI_REFBASEALPHA                 FLAG0_LIGHT0_POINT
            *flags |= ShaderFlag0::FLAG0_LIGHT0_POINT;
        }
        if self.has_flag(CharaToonFlags::MultiFitting) {
            // #define FLAG0_MULTI_FITTING                      FLAG0_LIGHT0_SPOT
            *flags |= ShaderFlag0::FLAG0_LIGHT0_SPOT;
        }
        if self.has_flag(CharaToonFlags::ReflectionBackground) {
            // #define FLAG0_REFLECTION_BACKGROUND              FLAG0_LIGHT2_DIRECTION
            *flags |= ShaderFlag0::FLAG0_LIGHT2_DIRECTION;
        }
        if self.has_flag(CharaToonFlags::ShadowHatchingDisable) {
            // #define FLAG1_SHADOWHATCHING_DISABLE             FLAG1_LIGHTMAP_MODULATE2
            *flags |= ShaderFlag1::FLAG1_LIGHTMAP_MODULATE2;
        } else if self.has_flag(CharaToonFlags::ShadowHatchingRefAlphaBaseColor) {
            // #define FLAG0_SHADOWHATCHING_REF_ALPHA_BASECOLOR FLAG0_LIGHT1_DIRECTION
            *flags |= ShaderFlag0::FLAG0_LIGHT1_DIRECTION;
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
        MaterialId::CharacterToon
    }
    fn get_shader_id(&self) -> u32 {
        3
    }
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for Toon
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Toon = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl Toon {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.shadow_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.edge_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.emissive_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.specular_color = RGBFloat::stream_read(stream, &mut ())?.into_raw();
        self.specular_power = stream.read_f32()?;
        self.metallic = stream.read_f32()?;
        self.edge_threshold = stream.read_f32()?;
        self.edge_factor = stream.read_f32()?;
        self.shadow_threshold = stream.read_f32()?;
        self.shadow_factor = stream.read_f32()?;
        self.flags = ToonBaseFlags::from_bits_truncate(stream.read_u32()?);
        self.field74 = stream.has_feature(GfdVersion::MaterialParameterToonSetP12).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.field7c = stream.has_feature(GfdVersion::MaterialParameterToonSetP12).map_or::<Result<Vec3, Box<dyn Error>>, _>(Ok(Vec3::ZERO), |_| Ok(Vec3::stream_read(stream, &mut ())?.into_raw()))?;
        self.bloom_strength = stream.has_feature(GfdVersion::MaterialParameterAddBloomIntensity).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.5), |_| Ok(stream.read_f32()?))?;
        self.specular_threshold = stream.has_feature(GfdVersion::MaterialParameterToonAddSpecularThreshold).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.edge_remove_y_axis_factor = stream.has_feature(GfdVersion::MaterialParameterToonAddEdgeRemoveYAxisFactor).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(3.), |_| Ok(stream.read_f32()?))?;
        self.field88 = stream.has_feature(GfdVersion::MaterialParameterToonAddP17).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.field8c = stream.has_feature(GfdVersion::MaterialParameterToonAddP17).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(-1.), |_| Ok(stream.read_f32()?))?;
        self.field90 = stream.has_feature(GfdVersion::MaterialParameterToonAddP17).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.field78 = stream.has_feature(GfdVersion::MaterialParameterToonAddP20).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.1), |_| Ok(stream.read_f32()?))?;
        self.roughness = stream.has_feature(GfdVersion::MaterialParameterToonAddMatRoughness).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.field94 = 1.;
        self.fitting_tile = stream.has_feature(GfdVersion::MaterialParameterToonAddFittingTile).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        self.multi_fitting_tile = stream.has_feature(GfdVersion::MaterialParameterToonAddMultiFittingTile).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.), |_| Ok(stream.read_f32()?))?;
        Ok(())
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for CharacterToon<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let _impl = Toon::stream_read(stream, &mut ())?.into_raw();
        Ok(CharacterToon { _impl, _alloc: PhantomData::<AObject> }.into())
    }
}