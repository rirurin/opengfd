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
use glam::Vec3;

// from https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

#[repr(C)]
#[derive(Debug)]
pub struct Layer {
    tile_size: f32,
    field1: f32,
    tile_offset: f32,
    field3: f32,
    roughness: f32,
    metallic: f32,
    color: Vec3
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Type15Flags : u32 {
        const TriplanarMapping = 0x00000001;
        const GBufferSkyFlag = 0x00000002;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Type15<A = GfdAllocator> 
where A: Allocator + Clone 
{
    layers: [Layer; 16],
    layer_count: u32,
    triplanar_scale: f32,
    flags: Type15Flags,
    _allocator: std::marker::PhantomData<A>
}


/// Source file: 49.HLSL


impl<A> Type15<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Type15<A> 
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
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        if self.flags.contains(Type15Flags::TriplanarMapping) {
            // #define FLAG2_TRIPLANARMAPPING FLAG2_HDR_TONEMAP
            *flags |=  ShaderFlag2::FLAG2_HDR_TONEMAP
        }
        if self.flags.contains(Type15Flags::GBufferSkyFlag) {
            *flags |=  ShaderFlag2::FLAG2_SKY
        }
    }
    fn update(&mut self) {
    }
}