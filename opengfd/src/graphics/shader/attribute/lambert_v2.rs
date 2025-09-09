use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
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
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};
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

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Lambert<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Lambert<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Lambert<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.ambient_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.diffuse_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.specular_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.emissive_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.reflectivity = stream.read_f32()?;
        self.lerp_blend_rate = stream.read_f32()?;
        Ok(())
    }
}