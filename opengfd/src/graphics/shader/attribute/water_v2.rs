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
use glam::Vec4;

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

    fn set_shader_flags(&self, vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
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
}