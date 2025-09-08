use allocator_api2::alloc::Allocator;
use crate::{
    graphics::{
        material::{ 
            Material, 
            MaterialType, 
        },
        shader::shader::ShaderFlags
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags,
};
use glam::Vec4;
use crate::utility::misc::RGBAFloat;
// From https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader file: 3.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct Lambert<A = GfdAllocator> 
where A: Allocator + Clone 
{
    ambient_color: RGBAFloat,
    diffuse_color: RGBAFloat,
    specular_color: RGBAFloat,
    emissive_color: RGBAFloat,
    reflectivity: f32,
    lerp_blend_rate: f32,
    _allocator: std::marker::PhantomData<A>
}

impl<A> Lambert<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Lambert<A> 
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
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }
    fn get_tex6_name(&self) -> &'static str { "Emissive Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, _flags: &mut ShaderFlags) {
        // TODO
    }
    fn update(&mut self) {
        /* 
        if self.flags.contains(FieldFlags::RemoveDiffuseShadow) {
            // TODO: Remove diffuse shadow
        }
        */
    }
}