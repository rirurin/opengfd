use std::error::Error;
use std::fmt::Debug;
use std::io::{Read, Seek, Write};
use std::marker::PhantomData;
use allocator_api2::alloc::Allocator;
use crate::{
    graphics::{
        material::{  
            Material, 
            MaterialType, 
        },
        shader::{
            attribute::toon_v2::Toon,
            shader::ShaderFlags
        }
    },
    kernel::allocator::GfdAllocator,
    object::geometry::VertexAttributeFlags,
};
use crate::utility::stream::{DeserializationStack, GfdSerialize, Stream, StreamIODevice};
use bitflags::bitflags;
use crate::graphics::material::params::MaterialId;
use crate::graphics::shader::attribute::toon_v2::ToonBaseFlags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord )]
    pub struct Type13Flags : u32 {
        const ToonRefNormalMap = 0x00000001;
        const ToonRemoveLightYAxis = 0x00000002;
        const EdgeRefNormalMap = 0x00000004;
        const Flag3 = 0x00000008;
        const Flag4 = 0x00000010;
        const EdgeRefNormalAlpha = 0x00000020;
        const EdgeSemitrans = 0x00000040;
        const EdgeRemoveLightYAxis = 0x00000080;
        const Flag8 = 0x00000100;
        const SubsurfaceScatterReceiver = 0x00000200;
        const Punchthrough = 0x00000400;
        const Flag11 = 0x00000800;
        const Flag12 = 0x00001000;
        const ApplyPBRLight = 0x00002000;
        const ForcedBloomIntensity = 0x00004000;
        const Flag15 = 0x00008000;
        const RefShadowColorAlpha = 0x00010000;
        const MultiLayerBaseTexture = 0x00020000;
        const MultiForeground = 0x00040000;
        const Fitting= 0x00080000;
        const MultiRefBaseAlpha = 1 << 20;
        const MultiFitting = 1 << 21;
        const ReflectionBackground = 1 << 22;
        const ShadowHatchingDisable = 1 << 23;
        const ShadowHatchingRefAlphaBaseColor = 1 << 24;
    }
}

pub struct Type13<A = GfdAllocator>
where A: Allocator + Clone
{
    _impl: Toon,
    _alloc: std::marker::PhantomData<A>
}

impl<A> Type13<A> 
where A: Allocator + Clone
{
    pub fn get_material(&self) -> &Material<A> {
        let ofs = Material::<A>::get_mat_data_offset();
        unsafe { &*((&raw const *self as *const u8).sub(ofs) as *const Material<A>) }
    }
    pub fn has_flag(&self, flag: Type13Flags) -> bool {
        self._impl.flags.contains(ToonBaseFlags::from_bits_truncate(flag.bits()))
    }
}

impl<A> MaterialType for Type13<A> 
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
        if self.has_flag(Type13Flags::Punchthrough) {
            return false;
        }
        !(self._impl.base_color.get_alpha_f32() >= 1. && self.get_material().get_constant() as i8 == -1)
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

    fn set_shader_flags(&self, _vtx: VertexAttributeFlags, _flags: &mut ShaderFlags) {
    }
    fn update(&mut self) {
        /* 
        if self.flags.contains(FieldFlags::RemoveDiffuseShadow) {
            // TODO: Remove diffuse shadow
        }
        */
    }
    fn get_material_id(&self) -> MaterialId {
        MaterialId::Type13
    }
    fn get_shader_id(&self) -> u32 {
        0xbc
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T> for Type13<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let _impl = Toon::stream_read(stream, &mut ())?.into_raw();
        Ok(Type13 { _impl, _alloc: PhantomData::<AObject> }.into())
    }
}