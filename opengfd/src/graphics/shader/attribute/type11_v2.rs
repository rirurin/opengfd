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
use crate::graphics::material::MaterialFlags;
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};
// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader File: 39.HLSL or 41.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct Type11<A = GfdAllocator> 
where A: Allocator + Clone
{
    base_color: RGBAFloat,
    field10: f32,
    _allocator: std::marker::PhantomData<A>
}

impl<A> Type11<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Type11<A> 
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
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, _flags: &mut ShaderFlags) {
    }
    fn update(&mut self) {
    }
    fn get_shader_id(&self) -> u32 {
        if !self.get_material().get_flag().contains(MaterialFlags::Light | MaterialFlags::Outline) || self.check_translucency() {
            0xb0
        } else {
            0xb1
        }
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Type11<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Type11<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Type11<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.field10 = stream.has_feature(GfdVersion::MaterialParameterType11AddField).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.5), |_| Ok(stream.read_f32()?))?;
        Ok(())
    }
}