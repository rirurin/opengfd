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
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SkyFlags: u32 {
        const BlendClearColor = 0x00000001;
    }
}

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader File: 37.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct Sky<A = GfdAllocator> 
where A: Allocator + Clone
{
    base_color: RGBAFloat,
    p10_2: f32,
    p10_3: f32,
    flags: SkyFlags,
    _allocator: std::marker::PhantomData<A>
}

impl<A> Sky<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Sky<A> 
where A: Allocator + Clone
{
    fn check_billboard_shadow_map(&self) -> bool {
        false
    }
    fn check_inside_14110ba40(&self) -> bool {
        false
    }
    fn check_invisible(&self) -> bool {
        self.base_color.get_alpha_f32() == 0.
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
        self.base_color.get_alpha_f32()
    }
    fn get_shadow_link_func(&self) -> u8 {
        0
    }
    fn get_tex1_name(&self) -> &'static str { "Base Texture" }
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        if self.flags.contains(SkyFlags::BlendClearColor) {
            *flags |= ShaderFlag2::FLAG2_BLEND_CLEARCOLOR;
        }
        // TODO: Transparency
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
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Sky<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Sky<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Sky<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.p10_2 = stream.has_feature(GfdVersion::MaterialParameterSkyAddP2).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.p10_3 = stream.read_f32()?;
        self.flags = stream.has_feature(GfdVersion::MaterialParameterSkyAddFlags).map_or::<Result<SkyFlags, Box<dyn Error>>, _>(Ok(SkyFlags::empty()), |_| Ok(SkyFlags::from_bits_retain(stream.read_u32()?)))?;
        Ok(())
    }
}