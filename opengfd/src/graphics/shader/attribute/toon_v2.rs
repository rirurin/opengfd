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
use glam::{ Vec3A, Vec4 };

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
    base_color: Vec4,
    shadow_color: Vec4,
    edge_color: Vec4,
    emissive_color: Vec4,
    specular_color: Vec3A,
    specular_power: f32,
    metallic: f32,
    edge_threshold: f32,
    edge_factor: f32,
    shadow_threshold: f32,
    shadow_factor: f32,
    field100: [u8; 0x38],
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
        const Flag10 = 0x00000400;
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
        if self._impl.shadow_threshold > 0.
        && self._impl.edge_color.w > 0. {
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
        // TODO: conditions for this
        *flags |= ShaderFlag1::FLAG1_MATERIAL_SPECULAR;
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
}