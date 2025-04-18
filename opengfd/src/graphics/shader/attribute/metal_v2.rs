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
    base_color: Vec4,
    shadow_color: Vec4,
    edge_color: Vec4,
    emissive_color: Vec4,
    specular_color: Vec4, // A: threshold
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
    p12_17: f32,
    mat_bloom_intensity2: f32,
    p12_8: Vec3,
    p12_11: f32,
    p12_12: f32,
    p12_13: f32,
    field128: u32,
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
        self.base_color.w == 0.
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
        && self.edge_color.w != 0. {
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