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
            MaterialFlags,
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
    pub struct FieldFlags: u32 {
        const Flag0 = 0x00000001;
        const Flag1 = 0x00000002;
        const Flag2 = 0x00000004;
        const Flag3 = 0x00000008;
        const Flag4 = 0x00000010;
        const InfluencedBySky = 0x00000020;
        const Transparency = 0x00000040;
        const MultiTextureMask = 0x00000080;
        const RemoveDiffuseShadow = 0x00000100;
        const BillboardShadowMap = 0x00000200;
        const Flag10 = 0x00000400;
        const Flag11 = 0x00000800;
    }
}

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader File: 7.HLSL or 9.HLSL
#[repr(C)]
#[derive(Debug)]
pub struct Field<A = GfdAllocator> 
where A: Allocator + Clone
{
    base_color: RGBAFloat,
    emissive_strength: f32,
    roughness: f32,
    metallic: f32,
    multi_alpha: f32,
    bloom_intensity: f32,
    flags: FieldFlags,
    _allocator: std::marker::PhantomData<A>
}

impl<A> Field<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for Field<A> 
where A: Allocator + Clone
{
    fn check_billboard_shadow_map(&self) -> bool {
        self.flags.contains(FieldFlags::BillboardShadowMap)
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
        self.flags.contains(FieldFlags::Transparency)
    }
    fn check_transparent_14107980(&self) -> bool {
        // let mat = self.get_material();
        if self.flags.contains(FieldFlags::Transparency) {
            self.get_base_color_opacity() != 1.
        } else {
            false
        }
    }
    fn get_base_color_opacity(&self) -> f32 {
        self.base_color.get_alpha_f32()
    }
    fn get_shadow_link_func(&self) -> u8 {
        0
    }
    fn get_tex1_name(&self) -> &'static str { "Base Texture" }
    fn get_tex2_name(&self) -> &'static str { "Normal Texture" }
    fn get_tex5_name(&self) -> &'static str { "Multiply Texture" }
    // R: roughness, G: metallic, B: emissive, A: intensity
    fn get_tex8_name(&self) -> &'static str { "PBR Params Texture" }

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        let mat = self.get_material();
        if self.flags.contains(FieldFlags::InfluencedBySky) {
            *flags |= ShaderFlag2::FLAG2_SKY;
        }
        if self.flags.contains(FieldFlags::InfluencedBySky)
        && mat.has_flags(MaterialFlags::Texture5) {
            *flags |= ShaderFlag2::FLAG2_MULTITEXTURE_AS_MASK;
        }
        if self.flags.contains(FieldFlags::RemoveDiffuseShadow) {
            // #define FLAG2_REMOVAL_DIFFUSESHADOW FLAG2_HDR_STAR
            *flags |= ShaderFlag2::FLAG2_HDR_STAR;
        }
        if self.flags.contains(FieldFlags::BillboardShadowMap) {
            // #define FLAG2_BILLBOARD_SHADOWMAP   FLAG2_SPECULAR_NORMALMAPALPHA
            *flags |= ShaderFlag2::FLAG2_SPECULAR_NORMALMAPALPHA;
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
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Field<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Field<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Field<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.emissive_strength = stream.read_f32()?;
        self.roughness = stream.read_f32()?;
        self.metallic = stream.read_f32()?;
        self.multi_alpha = stream.has_feature(GfdVersion::MaterialParameter0AddMultiAlpha).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(1.), |_| Ok(stream.read_f32()?))?;
        self.bloom_intensity = stream.has_feature(GfdVersion::MaterialParameterAddBloomIntensity).map_or::<Result<f32, Box<dyn Error>>, _>(Ok(0.5), |_| Ok(stream.read_f32()?))?;
        self.flags = stream.has_feature(GfdVersion::MaterialFieldAddFlags).map_or::<Result<FieldFlags, Box<dyn Error>>, _>(Ok(FieldFlags::empty()), |_| Ok(FieldFlags::from_bits_truncate(stream.read_u32()?)))?;
        if stream.get_header_version() == GfdVersion::MaterialFieldAddExtraFloat as u32 {
            let _ = stream.read_f32()?;
        }
        Ok(())
    }
}