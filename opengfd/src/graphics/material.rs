use bitflags::bitflags;
use crate::{
    graphics::{
        shader::shader::ShaderID,
        texture::Texture
    },
    utility::name::Name
};
use glam::Mat4;
use riri_mod_tools_proc::ensure_layout;

#[ensure_layout(size = 7usize)]
pub struct BlendType {
    #[field_offset(0usize)]
    pub type_: u8,
    #[field_offset(1usize)]
    pub src_color: u8,
    #[field_offset(2usize)]
    pub dst_color: u8,
    #[field_offset(3usize)]
    pub src_alpha: u8,
    #[field_offset(4usize)]
    pub dst_alpha: u8,
    #[field_offset(5usize)]
    pub multiple: u8,
    #[field_offset(6usize)]
    pub control: u8,
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
        const Flag7                    = 1 << 7;
        const Flag8                    = 1 << 8;
        const Flag9                    = 1 << 9;
        const FogDisable              = 1 << 10;
        const ShadowDisable           = 1 << 11;
        const Flag12                   = 1 << 12;
    }
}

#[ensure_layout(size = 1554usize)]
#[allow(non_snake_case)]
pub struct Material {
    #[field_offset(0usize)]
    pub blend: BlendType,
    #[field_offset(8usize)]
    pub culling: i16,
    #[field_offset(10usize)]
    pub dirty: u16,
    #[field_offset(12usize)]
    pub flags: MaterialFlags,
    #[field_offset(16usize)]
    pub texture: *mut MaterialTexture,
    #[field_offset(24usize)]
    pub shader: ShaderID,
    #[field_offset(56usize)]
    pub vertex_shader: usize,
    // pub vertexShader: *mut ngrVertexShaderWrapper,
    #[field_offset(64usize)]
    pub pixel_shader: usize,
    // pub pixelShader: *mut ngrPixelShaderWrapper,
    #[field_offset(72usize)]
    pub name: Name,
    #[field_offset(96usize)]
    pub alphaTestRef: i16,
    #[field_offset(98usize)]
    pub alphaTestFunc: i16,
    #[field_offset(100usize)]
    pub flags2: MaterialFlags2,
    #[field_offset(104usize)]
    pub sortPriority: i16,
    #[field_offset(106usize)]
    pub constant: i32,
    #[field_offset(110usize)]
    pub field16_0x6c: f32,
    #[field_offset(114usize)]
    pub bindCmd: *mut ::std::os::raw::c_void,
    #[field_offset(122usize)]
    pub unbindCmd: *mut ::std::os::raw::c_void,
    #[field_offset(130usize)]
    pub pixelBuffer: *mut ::std::os::raw::c_void,
    #[field_offset(138usize)]
    pub field20_0x88: *mut ::std::os::raw::c_void,
    #[field_offset(146usize)]
    pub data: MaterialData,
    // pub data: [u8; 588usize],
    #[field_offset(734usize)]
    pub mapType: u16,
    #[field_offset(736usize)]
    pub field23_0x2de: u16,
    #[field_offset(738usize)]
    pub ref_: i32,
    #[field_offset(742usize)]
    pub field25_0x2e4: [u16; 3usize],
    #[field_offset(754usize)]
    pub textures: [MaterialTexture; 10usize],
}

#[ensure_layout(size = 80usize)]
pub struct MaterialTexture {
    #[field_offset(0usize)]
    pub tm: Mat4,
    #[field_offset(64usize)]
    pub texture: *mut Texture,
    #[field_offset(72usize)]
    pub flags: i32,
    #[field_offset(76usize)]
    pub min: u8,
    #[field_offset(77usize)]
    pub mag: u8,
    #[field_offset(78usize)]
    pub wraps: u8,
    #[field_offset(79usize)]
    pub wrapt: u8,
}

#[allow(dead_code)]
// TODO: Material attribute types
pub union MaterialData {
    raw: [u8; 588]
}
