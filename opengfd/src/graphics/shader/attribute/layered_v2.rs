use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    graphics::{
        material::{ Material, MaterialType },
        shader::{
            flag::Flags2 as ShaderFlag2,
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags
};
use glam::Vec4;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TwoLayerFlags: u32 {
            const Automatic = 0x00000001;
            const UseSecondColorSet = 0x00000002;
            const TriplanarMappingLayer0 = 0x00000004;
            const TriplanarMappingLayer1 = 0x00000008;
            const TriplanarMappingBlend = 0x00000010;
            const Sky = 0x00000020;
            const FLAG6 = 0x00000040;
    }
}

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader file: 27.HLSL or 29.HLSL

#[repr(C)]
#[derive(Debug)]
pub struct LayerData {
    base_color: Vec4,
    emissive: f32,
    roughness: f32,
    bloom_intensity: f32,
    f4: f32
}

#[repr(C)]
#[derive(Debug)]
pub struct TwoLayer<A = GfdAllocator> 
where A: Allocator + Clone
{
    layers: [LayerData; 2],
    p6_1: f32,
    p6_2: f32,
    p6_3: f32,
    p6_4: f32,
    flags: TwoLayerFlags,
    _allocator: A
}

impl<A> TwoLayer<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for TwoLayer<A> 
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
        // let mat = self.get_material();
        if self.flags.contains(TwoLayerFlags::Automatic) {
            // #define FLAG2_TRIPLANARMAPPING_LAYER0 FLAG2_HDR_TONEMAP
            // #define FLAG2_TRIPLANARMAPPING_LAYER1 FLAG2_EDGE_CAVERNMAP
            // #define FLAG2_TRIPLANARMAPPING_BLEND  FLAG2_EDGE_REFERENCE_NORMALALPHA
            *flags |= ShaderFlag2::FLAG2_HDR_TONEMAP;
            *flags |= ShaderFlag2::FLAG2_EDGE_CAVERNMAP;
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALALPHA;
        } else {
            if self.flags.contains(TwoLayerFlags::TriplanarMappingLayer0) {
                *flags |= ShaderFlag2::FLAG2_HDR_TONEMAP;
            }
            if self.flags.contains(TwoLayerFlags::TriplanarMappingLayer1) {
                *flags |= ShaderFlag2::FLAG2_EDGE_CAVERNMAP;
            }
            if self.flags.contains(TwoLayerFlags::TriplanarMappingBlend) {
                *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALALPHA;
            }
        }
        if self.flags.contains(TwoLayerFlags::UseSecondColorSet) {
            // #define FLAG2_USECOLORSET2            FLAG2_HDR_STAR
            *flags |= ShaderFlag2::FLAG2_HDR_STAR;
        }
        if self.flags.contains(TwoLayerFlags::Sky) {
            *flags |= ShaderFlag2::FLAG2_SKY;
        }
        // TODO: Punchthrough 
    }
    fn update(&mut self) {
        /* 
        if self.flags.contains(FieldFlags::RemoveDiffuseShadow) {
            // TODO: Remove diffuse shadow
        }
        */
    }
}