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
use glam::Vec3;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};
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
        self.tile_size = stream.read_f32()?;
        self.field1 = stream.read_f32()?;
        self.tile_offset = stream.read_f32()?;
        self.field3 = stream.read_f32()?;
        self.roughness = stream.read_f32()?;
        self.metallic = stream.read_f32()?;
        self.color = Vec3::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
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

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Type15<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Type15<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Type15<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        for i in 0..16 {
            self.layers[i] = Layer::stream_read(stream, &mut ())?.into_raw();
        }
        self.layer_count = stream.read_u32()?;
        self.triplanar_scale = stream.read_f32()?;
        self.flags = Type15Flags::from_bits_truncate(stream.read_u32()?);
        Ok(())
    }
}