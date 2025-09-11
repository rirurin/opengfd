use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
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
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};
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
    field00: f32,
    field04: f32,
    field08: f32,
    field0c: f32,
    field10: f32,
    field14: f32,
}


#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for Layer
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Layer = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl Layer {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.field00 = stream.read_f32()?;
        self.field04 = stream.read_f32()?;
        self.field08 = stream.read_f32()?;
        self.field0c = stream.read_f32()?;
        self.field10 = stream.read_f32()?;
        self.field14 = stream.read_f32()?;
        Ok(())
    }
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
    fn get_shader_id(&self) -> u32 {
        0x98
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for FourLayer<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: FourLayer<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> FourLayer<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        for i in 0..4 {
            self.layers[i] = Layer::stream_read(stream, &mut ())?.into_raw();
        }
        self.p7_1 = stream.read_f32()?;
        self.flags = Type7Flags::from_bits_truncate(stream.read_u32()?);
        Ok(())
    }
}