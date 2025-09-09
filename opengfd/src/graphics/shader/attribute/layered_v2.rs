use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::mem::MaybeUninit;
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    graphics::{
        material::{ Material, MaterialType },
        shader::{
            flag::Flags2 as ShaderFlag2,
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags
};
use crate::kernel::version::GfdVersion;
use crate::utility::misc::RGBAFloat;
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TwoLayerFlags: u32 {
            const Automatic = 0x00000001;
            const UseSecondColorSet = 0x00000002;
            const TriplanarMappingLayer0 = 0x00000004;
            const TriplanarMappingLayer1 = 0x00000008;
            const TriplanarMappingBlend = 0x00000010;
            const Sky = 0x00000020;
            const FLAG6 = 0x00000040;
    }
}

// See https://github.com/tge-was-taken/GFD-Studio/blob/master/GFDLibrary/Materials/MaterialParameterSet_Metaphor.cs

/// Shader file: 27.HLSL or 29.HLSL

#[repr(C)]
#[derive(Debug)]
pub struct LayerData {
    base_color: RGBAFloat,
    emissive: f32,
    roughness: f32,
    bloom_intensity: f32,
    f4: f32
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for LayerData
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: LayerData = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl LayerData {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.base_color = RGBAFloat::stream_read(stream, &mut ())?.into_raw();
        self.emissive = stream.read_f32()?;
        self.roughness = stream.read_f32()?;
        self.bloom_intensity = stream.read_f32()?;
        self.f4 = stream.read_f32()?;
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct TwoLayer<A = GfdAllocator> 
where A: Allocator + Clone
{
    layers: [LayerData; 2],
    p6_1: f32,
    p6_2: f32,
    p6_3: f32,
    p6_4: f32,
    flags: TwoLayerFlags,
    _allocator: std::marker::PhantomData<A>
}

impl<A> TwoLayer<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
}

impl<A> MaterialType for TwoLayer<A> 
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

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        // let mat = self.get_material();
        if self.flags.contains(TwoLayerFlags::Automatic) {
            // #define FLAG2_TRIPLANARMAPPING_LAYER0 FLAG2_HDR_TONEMAP
            // #define FLAG2_TRIPLANARMAPPING_LAYER1 FLAG2_EDGE_CAVERNMAP
            // #define FLAG2_TRIPLANARMAPPING_BLEND  FLAG2_EDGE_REFERENCE_NORMALALPHA
            *flags |= ShaderFlag2::FLAG2_HDR_TONEMAP;
            *flags |= ShaderFlag2::FLAG2_EDGE_CAVERNMAP;
            *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALALPHA;
        } else {
            if self.flags.contains(TwoLayerFlags::TriplanarMappingLayer0) {
                *flags |= ShaderFlag2::FLAG2_HDR_TONEMAP;
            }
            if self.flags.contains(TwoLayerFlags::TriplanarMappingLayer1) {
                *flags |= ShaderFlag2::FLAG2_EDGE_CAVERNMAP;
            }
            if self.flags.contains(TwoLayerFlags::TriplanarMappingBlend) {
                *flags |= ShaderFlag2::FLAG2_EDGE_REFERENCE_NORMALALPHA;
            }
        }
        if self.flags.contains(TwoLayerFlags::UseSecondColorSet) {
            // #define FLAG2_USECOLORSET2            FLAG2_HDR_STAR
            *flags |= ShaderFlag2::FLAG2_HDR_STAR;
        }
        if self.flags.contains(TwoLayerFlags::Sky) {
            *flags |= ShaderFlag2::FLAG2_SKY;
        }
        // TODO: Punchthrough 
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
impl<AStream, AObject, T> GfdSerialize<AStream, T> for TwoLayer<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: TwoLayer<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> TwoLayer<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        for i in 0..2 {
            self.layers[i] = LayerData::stream_read(stream, &mut ())?.into_raw();
        }
        if stream.get_header_version() < GfdVersion::MaterialParameterLayerExtraFields as u32 {
            let value = stream.read_f32()?;
            self.p6_1 = value;
            self.p6_2 = value;
        } else {
            self.p6_1 = stream.read_f32()?;
            self.p6_2 = stream.read_f32()?;
            self.p6_3 = stream.read_f32()?;
        }
        self.p6_4 = stream.read_f32()?;
        self.flags = TwoLayerFlags::from_bits_truncate(stream.read_u32()?);
        Ok(())
    }
}