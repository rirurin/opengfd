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
use crate::utility::misc::{RGBAFloat, RGBFloat};
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};
// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader File: 39.HLSL or 41.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct Type9<A = GfdAllocator> 
where A: Allocator + Clone
{
    field0: f32,
    field4: f32,
    field8: f32,
    fieldc: f32,
    field10: RGBAFloat,
    field20: RGBAFloat,
    field30: RGBAFloat,
    field40: RGBAFloat,
    field50: RGBFloat,
    field5c: f32,
    field60: f32,
    field64: f32,
    field68: f32,
    field6c: f32,
    field70: f32,
    field74: f32,
    field78: f32,
    field7c: u32,
    _allocator: std::marker::PhantomData<A>
}

impl<A> Type9<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Type9<A> 
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

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, _flags: &mut ShaderFlags) {
    }
    fn update(&mut self) {
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Type9<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Type9<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Type9<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.field0 = stream.read_f32()?;
        self.field4 = stream.read_f32()?;
        self.field8 = stream.read_f32()?;
        self.fieldc = stream.read_f32()?;
        self.field10 = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.field20 = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.field30 = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.field40 = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.field50 = RGBFloat::stream_read(stream, &mut ())?.into_raw();
        self.field60 = stream.read_f32()?;
        self.field64 = stream.read_f32()?;
        self.field68 = stream.read_f32()?;
        self.field6c = stream.read_f32()?;
        self.field70 = stream.read_f32()?;
        self.field74 = stream.read_f32()?;
        self.field78 = stream.read_f32()?;
        self.field7c = stream.read_u32()?;
        Ok(())
    }
}