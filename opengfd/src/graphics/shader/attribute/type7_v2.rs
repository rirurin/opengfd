use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    graphics::{
        material::{ 
            Material, 
            MaterialType, 
        },
        shader::{
            flag::Flags2 as ShaderFlag2,
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags,
};

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader File: 29.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct FourLayer<A = GfdAllocator> 
where A: Allocator + Clone
{
    layers: [Layer; 4],
    p7_1: f32,
    flags: Type7Flags,
    _allocator: std::marker::PhantomData<A>
}

#[repr(C)]
#[derive(Debug)]
pub struct Layer {
    data: [f32; 6]
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Type7Flags : u32 {
        const TriplanarMapping = 1 << 0;
        const Sky = 1 << 1;
    }
}

impl<A> FourLayer<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for FourLayer<A> 
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
    fn get_tex1_name(&self) -> &'static str { "Layer 0 Base Texture" }
    fn get_tex2_name(&self) -> &'static str { "Layer 0 Normal Texture" }
    fn get_tex3_name(&self) -> &'static str { "Layer 1 Normal Texture" }
    fn get_tex6_name(&self) -> &'static str { "Layer 1 Base Texture" }
    fn get_tex7_name(&self) -> &'static str { "Layer 2 Base Texture" }
    fn get_tex8_name(&self) -> &'static str { "Layer 2 Normal Texture" }
    fn get_tex9_name(&self) -> &'static str { "Layer 3 Base Texture" }
    fn get_tex10_name(&self) -> &'static str { "Layer 3 Normal Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        // #define FLAG2_TRIPLANARMAPPING FLAG2_HDR_TONEMAP
        if self.flags.contains(Type7Flags::TriplanarMapping) {
            *flags |= ShaderFlag2::FLAG2_HDR_TONEMAP;
        }
        if self.flags.contains(Type7Flags::Sky) {
            *flags |= ShaderFlag2::FLAG2_SKY;
        }
    }
    fn update(&mut self) {
    }
}