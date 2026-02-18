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
        shader::{
            flag::{
                Flags0 as ShaderFlag0,
                Flags1 as ShaderFlag1,
                Flags2 as ShaderFlag2
            },
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags,
};
use crate::utility::misc::{RGBAFloat, RGBFloat};
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};
use bitflags::bitflags;
use crate::graphics::material::params::MaterialId;
// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Type9Flags: u32 {
        const ToonRefNormalMap = 1 << 0;
        const ToonRemoveLightYAxis = 1 << 1;
        const EdgeRefNormalMap = 1 << 2;
        const HasOutline = 1 << 3;
        const FLAG4 = 1 << 4;
        const EdgeReferenceNormalAlpha = 1 << 5;
        const Semitrans = 1 << 6;
        const FLAG7 = 1 << 7;
        const FLAG8 = 1 << 8;
        const FLAG9 = 1 << 9;
        const FLAG10 = 1 << 10;
        const FLAG11 = 1 << 11;
        const FLAG12 = 1 << 12;
        const FLAG13 = 1 << 13;
        const FLAG14 = 1 << 14;
        const FLAG15 = 1 << 15;
    }
}

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
    base_color: RGBAFloat,
    shadow_color: RGBAFloat,
    edge_color: RGBAFloat,
    emissive_color: RGBAFloat,
    specular_color: RGBFloat,
    specular_threshold: f32,
    specular_power: f32,
    metallic: f32,
    roughness: f32,
    edge_threshold: f32,
    field70: f32,
    field74: f32,
    field78: f32,
    flags: Type9Flags,
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

    fn set_shader_flags(&self, vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        if self.metallic > 0. {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_REFLECTION;
        }
        if self.edge_threshold > 0. &&  self.edge_color.get_alpha_f32() > 0. {
            *flags |= ShaderFlag2::FLAG2_EDGE_BACKLIGHT;
        }
        if self.flags.contains(Type9Flags::ToonRefNormalMap) {
            *flags |= ShaderFlag2::FLAG2_TOON_REFERENCE_NORMALMAP;
        }
        if self.flags.contains(Type9Flags::ToonRemoveLightYAxis) {
            *flags |= ShaderFlag2::FLAG2_TOON_REMOVAL_LIGHT_YAXIS;
        }
        if self.flags.contains(Type9Flags::EdgeRefNormalMap) {
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALMAP;
        }
        if self.flags.contains(Type9Flags::EdgeReferenceNormalAlpha) {
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALALPHA;
        }
        if self.flags.contains(Type9Flags::Semitrans) {
            *flags |= ShaderFlag2::FLAG2_EDGE_SEMITRANS;
        }
        if self.specular_power > 0.
            && self.specular_threshold > 0.
            && self.specular_color != RGBFloat::default() {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_SPECULAR;
        }
    }
    fn update(&mut self) {
    }
    fn get_material_id(&self) -> MaterialId {
        MaterialId::Type9
    }
    fn get_shader_id(&self) -> u32 {
        0x9b
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
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.shadow_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.edge_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.emissive_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.specular_color = RGBFloat::stream_read(stream, &mut ())?.into_raw();
        self.specular_threshold = stream.read_f32()?;
        self.specular_power = stream.read_f32()?;
        self.metallic = stream.read_f32()?;
        self.roughness = stream.read_f32()?;
        self.edge_threshold = stream.read_f32()?;
        self.field70 = stream.read_f32()?;
        self.field74 = stream.read_f32()?;
        self.field78 = stream.read_f32()?;
        self.flags = Type9Flags::from_bits_retain(stream.read_u32()?);
        Ok(())
    }
}