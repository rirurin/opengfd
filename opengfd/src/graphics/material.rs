use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    graphics::{
        shader::{
            flag:: { 
                Flags0 as ShaderFlag0,
                Flags1 as ShaderFlag1,
                Flags2 as ShaderFlag2
            },
            shader::{ ShaderID, ShaderFlags }
        },
        texture::Texture
    },
    kernel::{
        allocator::GfdAllocator,
        global::GraphicsFlags
    },
    object::geometry::VertexAttributeFlags,
    utility::{
        name::Name,
        reference::{ GfdRcType, Reference }
    }
};
use glam::Mat4;
use opengfd_proc::GfdRcAuto;

// use std::{
//     error::Error,
//     fmt::Display
// };
// use riri_mod_tools_proc::ensure_layout;

#[repr(C)]
#[derive(Debug)]
pub struct Blending {
    ty: BlendType,
    src_color: u8,
    dst_color: u8,
    src_alpha: u8,
    dst_alpha: u8,
    multiple: MultiplyType,
    control: u8,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlendType {
    Opaque = 0,
    Semitrans = 1,
    AddTrans = 2,
    SubTrans = 3,
    ModulateTrans = 4,
    Modulate2Trans = 5,
    Advanced = 6
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiplyType {
    None = 0,
    Semi = 1,
    Add = 2,
    Mod = 3,
    None4 = 4,
    Sub = 5
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MaterialFlags : u32 {
        const Ambient             = 1 << 0; 
        const Diffuse             = 1 << 1;   
        const Specular            = 1 << 2;   
        const Transparency        = 1 << 3;   
        const VertexColor         = 1 << 4;   
        const Fog                 = 1 << 5;   
        const Diffusivity         = 1 << 6;   
        const UVTransform         = 1 << 7;
        const Emissive            = 1 << 8;   
        const Reflection          = 1 << 9;   
        const Shadow              = 1 << 10;
        const Light               = 1 << 11;  
        const Wireframe           = 1 << 12;  
        const AlphaTest           = 1 << 13;  
        const ShadowReceiver      = 1 << 14;  
        const ShadowCaster        = 1 << 15;  
        const Extension           = 1 << 16;  
        const Outline             = 1 << 17;  
        const SpNormalAlpha       = 1 << 18;  
        const ReflectionCaster    = 1 << 19;  
        const Texture1            = 1 << 20;  
        const Texture2            = 1 << 21;  
        const Texture3            = 1 << 22;  
        const Texture4            = 1 << 23;  
        const Texture5            = 1 << 24;  
        const Texture6            = 1 << 25;  
        const Texture7            = 1 << 26;  
        const Texture8            = 1 << 27;  
        const Texture9            = 1 << 28;  
        const Texture10           = 1 << 29;  
        const SSAO                = 1 << 30;  
        const ConstantColor       = 1 << 31;  
    }
}

bitflags! { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MaterialFlags2 : u16 {
        const EnableBloom             = 1 << 0;
        const LightMapModulateMode    = 1 << 1;
        const LightMapModulate2       = 1 << 2;
        const Flag3                    = 1 << 3;
        const DisableCharacterOutline = 1 << 5; 
        const Flag6 = 1 << 6;
        const ConstantColor = 1 << 7;
        const Flag8 = 1 << 8;
        const Grayscale = 1 << 9;
        const FogDisable              = 1 << 10;
        const ShadowDisable           = 1 << 11;
        const Flag12                   = 1 << 12;
    }
}

#[cfg(feature = "v1-core")]
pub mod params {
    // TODO: P5R Material Types
}

#[cfg(feature = "v2-core")]
pub mod params {
    use allocator_api2::alloc::Allocator;
    use crate::graphics::{
        material::{ Material, MaterialType },
        shader::attribute::{
            field_v2::Field,
            lambert_v2::Lambert,
            toon_v2::CharacterToon,
            type3_v2::Type3,
            distortion_v2::CharacterDistortion,
            water_v2::Water,
            layered_v2::TwoLayer,
            type7_v2::FourLayer,
            type8_v2::Type8,
            type9_v2::Type9,
            sky_v2::Sky,
            type11_v2::Type11,
            metal_v2::Metal,
            type13_v2::Type13,
            type14_v2::Type14,
            type15_v2::Type15,
            shadow_v2::Shadow
        }
    };
    use std::{
        error::Error,
        fmt::Display,
        mem::ManuallyDrop
    };

    #[repr(u16)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum MaterialId {
        Field = 0,
        Lambert = 1,
        CharacterToon = 2,
        Type3 = 3,
        CharacterDistort = 4,
        Water = 5,
        DualLayer = 6,
        Type7 = 7,
        Type8 = 8,
        Type9 = 9,
        Sky = 10,
        Type11 = 11,
        CharacterMetal = 12,
        Type13 = 13,
        Type14 = 14,
        Type15 = 15,
        Shadow = 16
    }

    #[allow(dead_code)]
    #[repr(C, packed(1))]
    pub union MaterialDataStorage {
        field: ManuallyDrop<Field>,
        lambert: ManuallyDrop<Lambert>,
        chara_toon: ManuallyDrop<CharacterToon>,
        type3: ManuallyDrop<Type3>,
        chara_distort: ManuallyDrop<CharacterDistortion>,
        water: ManuallyDrop<Water>,
        dual_layer: ManuallyDrop<TwoLayer>,
        type7: ManuallyDrop<FourLayer>,
        type8: ManuallyDrop<Type8>,
        type9: ManuallyDrop<Type9>,
        sky: ManuallyDrop<Sky>,
        type11: ManuallyDrop<Type11>,
        metal: ManuallyDrop<Metal>,
        type13: ManuallyDrop<Type13>,
        type14: ManuallyDrop<Type14>,
        type15: ManuallyDrop<Type15>,
        shadow: ManuallyDrop<Shadow>
    }

    #[derive(Debug)]
    pub struct MaterialIdMismatch(MaterialId, MaterialId);
    impl Error for MaterialIdMismatch {}
    impl Display for MaterialIdMismatch {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Expected material ID {:?}, got ID {:?} instead", self.0, self.1)
        }
    }

    impl<A> Material<A>
    where A: Allocator + Clone
    {
        pub fn get_specific_data<M>(&self) -> Result<&M, MaterialIdMismatch>
        where M: MaterialType {
            Err(MaterialIdMismatch(MaterialId::Field, MaterialId::Field))
        }
        pub fn get_data(&self) -> Box<&dyn MaterialType> {
            Box::new(match self.map_type {
                MaterialId::Field => unsafe { &*(&raw const self.data as *const Field<A>) },
                MaterialId::Lambert => unsafe { &*(&raw const self.data as *const Lambert<A>) },
                MaterialId::CharacterToon => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Type3 => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::CharacterDistort => unsafe { &*(&raw const self.data as *const CharacterDistortion<A>) },
                MaterialId::Water => unsafe { &*(&raw const self.data as *const Water<A>) },
                MaterialId::DualLayer => unsafe { &*(&raw const self.data as *const TwoLayer<A>) },
                MaterialId::Type7 => unsafe { &*(&raw const self.data as *const FourLayer<A>) },
                MaterialId::Type8 => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Type9 => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Sky => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Type11 => unsafe { &*(&raw const self.data as *const Type11<A>) },
                MaterialId::CharacterMetal => unsafe { &*(&raw const self.data as *const Metal<A>) },
                MaterialId::Type13 => unsafe { &*(&raw const self.data as *const Type13<A>) },
                MaterialId::Type14 => unsafe { &*(&raw const self.data as *const Type14<A>) },
                MaterialId::Type15 => unsafe { &*(&raw const self.data as *const Type15<A>) },
                MaterialId::Shadow => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
            })
        }
    }
}

#[repr(C)]
#[derive(GfdRcAuto)]
pub struct Material<A = GfdAllocator>
where A: Allocator + Clone
{
    blend: Blending,
    culling: Culling,
    dirty: u16,
    flags: MaterialFlags,
    texture: *mut MaterialTexture,
    shader: ShaderID,
    vertex_shader: usize,
    // pub vertexShader: *mut ngrVertexShaderWrapper,
    pixel_shader: usize,
    // pub pixelShader: *mut ngrPixelShaderWrapper,
    name: Name,
    alpha_test_ref: i16,
    alpha_test_func: AlphaTestFunc,
    flags2: MaterialFlags2,
    sort_priority: i16,
    constant: i32,
    field16_0x6c: f32,
    bind_cmd: *mut ::std::os::raw::c_void,
    unbind_cmd: *mut ::std::os::raw::c_void,
    pixel_buffer: *mut ::std::os::raw::c_void,
    field20_0x88: *mut ::std::os::raw::c_void,
    // data: MaterialData,
    data: params::MaterialDataStorage,
    map_type: params::MaterialId,
    field23_0x2de: u16,
    ref_: Reference,
    field25_0x2e4: [u16; 3usize],
    textures: [MaterialTexture; 10usize],
    _allocator: A,
}


#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlphaTestFunc {
    Never = 0,
    LessOrEqual0 = 1,
    Equal = 2,
    LessOrEqual1 = 3,
    GreaterOrEqual0 = 4,
    NotEqual = 5,
    GreaterOrEqual1 = 6
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Culling {
    Backface = 0,
    None = 1
}

#[repr(C)]
pub struct MaterialTexture {
    tm: Mat4,
    texture: *mut Texture,
    flags: MaterialTextureFlags,
    min: u8,
    mag: u8,
    wraps: u8,
    wrapt: u8,
}

bitflags! { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MaterialTextureFlags : u32 {
        const HasUVTransform = 1 << 0;
    }
}

impl<A> Material<A>
where A: Allocator + Clone
{
    /// Original function: gfdMaterialCheckAlphaTest
    pub fn check_alpha_test(&self) -> bool {
        self.flags.contains(MaterialFlags::AlphaTest)
    }
    /// Original function: gfdMaterialClearDirtyBit
    pub fn clear_dirty_bit(&mut self, dirty: u16) {
        self.dirty &= dirty;
    }
    /// Original function: gfdMaterialCheckShadowCaster
    pub fn check_shadow_caster(&self) -> bool {
        let glb = unsafe { crate::globals::get_gfd_global_unchecked() };
        self.flags.contains(MaterialFlags::ShadowCaster)
        && glb.graphics.has_flags(GraphicsFlags::ShadowCaster)

    }

    pub fn has_flags(&self, flag: MaterialFlags) -> bool {
        self.flags.contains(flag)
    }
    pub fn has_flags2(&self, flag: MaterialFlags2) -> bool {
        self.flags2.contains(flag)
    }
    pub fn has_any_flag(&self, flag: MaterialFlags) -> bool {
        self.flags.intersects(flag)
    }
    pub fn has_any_flag2(&self, flag: MaterialFlags2) -> bool {
        self.flags2.intersects(flag)
    }

    /// Original function: gfdMaterialClearFlagBit
    pub fn clear_flag_bit(&mut self, flags: MaterialFlags) {
        self.flags &= !flags;
    }
    /// Original function: gfdMaterialGetFlag
    pub fn get_flag(&self) -> MaterialFlags {
        self.flags
    }
    /// Original function: gfdMaterialGetFlag2
    pub fn get_flag2(&self) -> MaterialFlags2 {
        self.flags2
    }
    pub fn get_mat_data_offset() -> usize {
        std::mem::offset_of!(Self, data)
    }
    /// Original function: gfdMaterialSetFlagBit
    pub fn set_flag(&mut self, flag: MaterialFlags, value: bool) {
        self.flags.set(flag, value)
    }
    pub fn set_flag2(&mut self, flag: MaterialFlags2, value: bool) {
        self.flags2.set(flag, value)
    }
    #[inline(always)]
    pub fn flag_on(&mut self, flag: MaterialFlags) {
        self.set_flag(flag, true)
    }
    #[inline(always)]
    pub fn flag2_on(&mut self, flag: MaterialFlags2) {
        self.set_flag2(flag, true)
    }
    #[inline(always)]
    pub fn flag_off(&mut self, flag: MaterialFlags) {
        self.set_flag(flag, false)
    }
    #[inline(always)]
    pub fn flag2_off(&mut self, flag: MaterialFlags2) {
        self.set_flag2(flag, false)
    }
    /// Original function: gfdMaterialSetFlag
    pub fn set_all_flags(&mut self, flag: MaterialFlags) {
        self.flags = flag;
    }
    /// Original function: gfdMaterialSetFlag2
    pub fn set_all_flags2(&mut self, flag: MaterialFlags2) {
        self.flags2 = flag;
    }
}
pub trait MaterialType {
    fn check_billboard_shadow_map(&self) -> bool;
    fn check_inside_14110ba40(&self) -> bool;
    /// Original function: gfdMaterialCheckInvisible
    fn check_invisible(&self) -> bool;
    /// Original function: gfdMaterialCheckOutline
    fn check_outline(&self) -> bool;
    // For gfdGeometryGetRenderPriority
    fn check_render_prio_mod(&self) -> bool;
    fn check_subsurface_scatter(&self) -> bool;
    fn check_toon_flag_8000(&self) -> bool;
    /// Original function: gfdMaterialCheckTranslucently
    fn check_translucency(&self) -> bool;
    fn check_transparent_14107980(&self) -> bool;
    fn get_base_color_opacity(&self) -> f32;
    // Inside of gfdGeometryRender, after gfdMaterialCheckTranslucently
    fn get_shadow_link_func(&self) -> u8;

    fn get_tex1_name(&self) -> &'static str { "Texture 1" } // register(t0)
    fn get_tex2_name(&self) -> &'static str { "Texture 2" } // register(t1)
    fn get_tex3_name(&self) -> &'static str { "Texture 3" } // register(t2)
    fn get_tex4_name(&self) -> &'static str { "Texture 4" } // register(t3)
    fn get_tex5_name(&self) -> &'static str { "Texture 5" } // register(t4)
    fn get_tex6_name(&self) -> &'static str { "Texture 6" } // register(t5)
    fn get_tex7_name(&self) -> &'static str { "Texture 7" } // register(t6)
    fn get_tex8_name(&self) -> &'static str { "Texture 8" } // register(t7)
    fn get_tex9_name(&self) -> &'static str { "Texture 9" } // register(t8)
    fn get_tex10_name(&self) -> &'static str { "Texture 10" } // register(t9)

    // See gfdMaterialGetShaderFlags
    fn set_shader_flags(&self, vtx: VertexAttributeFlags, flags: &mut ShaderFlags);

    // fn set_ambient_color(&self, )
    // diffuse, emissive, specular, diffusivity, reflectivity
    /// Original function: gfdMaterialUpdate
    fn update(&mut self);
    // Material->MapType also called from 
    // - inside gfdThJobGeometryUpdate
    // TODO: StreamRead/StreamWrite, create material

}

const TEX_BIT_SIZE: usize = 3;
const TEX_UV_ID_MAX: usize = 2;
const TEX_BIT_MAX: usize = (1 << TEX_BIT_SIZE) - 1;

impl<A> Material<A>
where A: Allocator + Clone
{

    pub fn set_texture_map_flags(&self, tex_id: usize, flags: &mut ShaderFlags) {
        // const Texture1 = 1 << 20;
        let texflag = MaterialFlags::from_bits_retain(1 << (0x14 + tex_id));
        let texin = (self.shader.get_texcoord_in() as usize >> (tex_id * TEX_BIT_SIZE) & TEX_BIT_MAX ) as u32;
        let texout = (self.shader.get_texcoord_out() as usize >> (tex_id * TEX_BIT_SIZE) & TEX_BIT_MAX ) as u32;
        if self.flags.contains(texflag) && texin <= TEX_UV_ID_MAX as u32 {
            // const FLAG1_TEXTURE1 = 1 << 0x16;
            *flags |= ShaderFlag1::from_bits_retain(1 << (0x16 + tex_id));
            // const FLAG0_TEXCOORD0IN = 1 << 0x10 and const FLAG0_TEXCOORD0OUT = 1 << 0x13;
            *flags |= ShaderFlag0::from_bits_retain(1 << (0x10 + texin) | 1 << (0x13 + texout));
            if self.flags.contains(MaterialFlags::UVTransform) &&
            self.textures[tex_id].flags.contains(MaterialTextureFlags::HasUVTransform) {
                // const FLAG1_MATERIAL_UV0TRANSFORM = 1 << 0xc;
                *flags |= ShaderFlag1::from_bits_retain(1 << (0xc + texout));
                
            }
        }
    }

    // 0x141071d70
    pub fn get_shader_flags(&self, map_id: u16, vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        let param = self.get_data();
        let glb = unsafe { crate::globals::get_gfd_global_unchecked() };
        flags.reset_flag0(ShaderFlag0::FLAG0_ALWAYS_ENABLED);
        flags.reset_flag1(ShaderFlag1::FLAG1_MATERIAL_AMBDIFF);
        flags.reset_flag2(ShaderFlag2::empty());
        // Light info
        if self.flags.contains(MaterialFlags::Light) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_LIGHT;
            if self.flags.contains(MaterialFlags::Texture9) { // light map
                if !self.flags2.contains(MaterialFlags2::LightMapModulateMode) {
                    if self.flags2.contains(MaterialFlags2::LightMapModulate2) {
                        *flags |= ShaderFlag1::FLAG1_LIGHTMAP_MODULATE2;
                    }
                } else {
                    *flags |= ShaderFlag2::FLAG2_LIGHTMAP_MODULATE;
                }
            }
        }
        if self.flags.contains(MaterialFlags::AlphaTest) {
            *flags |= match self.alpha_test_func {
                AlphaTestFunc::Never => ShaderFlag2::FLAG2_ATEST_NEVER,
                AlphaTestFunc::LessOrEqual0 |
                AlphaTestFunc::LessOrEqual1 => ShaderFlag2::FLAG2_ATEST_LESS_LEQUAL,
                AlphaTestFunc::Equal => ShaderFlag2::FLAG2_ATEST_EQUAL,
                AlphaTestFunc::NotEqual => ShaderFlag2::FLAG2_ATEST_NOTEQUAL,
                AlphaTestFunc::GreaterOrEqual0 |
                AlphaTestFunc::GreaterOrEqual1 => ShaderFlag2::FLAG2_ATEST_GREATER_GEQUAL,
            };
        }
        if param.check_translucency() {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_TRANSPARENCY;
        } else if self.flags.contains(MaterialFlags::Diffusivity) {
            *flags |= ShaderFlag0::FLAG0_TEMPERARE;
        }
        if self.flags.contains(MaterialFlags::VertexColor) 
        && vtx.contains(VertexAttributeFlags::DiffuseColor) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_VERTEXCOLOR;
        }
        if self.flags.contains(MaterialFlags::Fog) {
            if glb.graphics.has_flags(GraphicsFlags::Fog) {
                *flags |= ShaderFlag1::FLAG1_MATERIAL_FOG;
            }
            if glb.graphics.has_flags(GraphicsFlags::HeightFog) {
                *flags |= ShaderFlag1::FLAG1_MATERIAL_HEIGHTFOG;
            }
            if self.blend.ty == BlendType::AddTrans
            || self.blend.ty == BlendType::Modulate2Trans {
                *flags |= ShaderFlag2::FLAG2_MATERIAL_MODULATE_FOG;
            }
        }
        if self.flags.contains(MaterialFlags::SSAO) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_OCCLUSION;
        }
        if self.flags.contains(MaterialFlags::Emissive) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_EMISSIVE;
        }
        if self.flags.contains(MaterialFlags::ShadowReceiver) 
        && glb.graphics.has_flags(GraphicsFlags::ShadowCaster) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_SHADOW;
        }
        if self.flags.contains(MaterialFlags::Texture5) {
            match self.blend.multiple {
                MultiplyType::Semi => {
                    *flags |= ShaderFlag2::FLAG2_MATERIAL_MULTIPLE_SEMI;
                },
                MultiplyType::Add => {
                    *flags |= ShaderFlag2::FLAG2_MATERIAL_MULTIPLE_ADD;
                },
                MultiplyType::Mod => {
                    *flags |= ShaderFlag1::FLAG1_MATERIAL_MULTIPLE_MOD;
                },
                MultiplyType::Sub => {
                    *flags |= ShaderFlag2::FLAG2_MATERIAL_MULTIPLE_SUB;
                },
                _ => {}
            }
        }
        for i in 0..10 {
            self.set_texture_map_flags(i, flags);
        }
        if self.flags.contains(MaterialFlags::SpNormalAlpha) {
            *flags |= ShaderFlag2::FLAG2_SPECULAR_NORMALMAPALPHA;
        }
        if map_id == 4 {
            *flags |= ShaderFlag0::FLAG0_INFINITE;
        }
        param.set_shader_flags(vtx, flags);
        if self.flags.contains(MaterialFlags::ReflectionCaster) {
            *flags |= ShaderFlag2::FLAG2_REFLECTION_CASTER;
        }
        if self.flags2.contains(MaterialFlags2::ConstantColor) {
            *flags |= ShaderFlag0::FLAG0_CONSTANTCOLOR;
        }
        if self.flags2.contains(MaterialFlags2::Grayscale) {
            *flags |= ShaderFlag0::FLAG0_CONSTANTCOLOR | ShaderFlag0::FLAG0_GRAYSCALE;
        }
    }
}