use std::ptr::NonNull;

// see https://github.com/angryzor/rangers-api/blob/22aa1b0c04c35b6871e1affbe0e98ab6e4586543/rangers-api/Hedgehog/Needle/Texture.h

pub(crate) static NEEDLE_RESOURCE_TEXTURE: u64 = 0x3045525554584554;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Usage {
    DEFAULT,
    IMMUTABLE,
    DYNAMIC,
    STAGING,
    IMMUTABLE2,
    DYNAMIC2,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum BindFlag {
    SHADER_RESOURCE,
    RENDER_TARGET,
    DEPTH_STENCIL,
    VERTEX_BUFFER,
    INDEX_BUFFER,
    CONSTANT_BUFFER,
    STREAM_OUTPUT,
    UNORDERED_ACCESS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum MiscFlag {
    GENERATE_MIPS,
    SHARED,
    TEXTURECUBE,
    UNK3,
    UNK4,
    BUFFER_STRUCTURED,
    UNK6,
    UNK7,
    UNK8,
    UNK9,
    DRAWINDIRECT_ARGS,
    BUFFER_ALLOW_RAW_VIEWS,
    UNK12,
    UNK13,
    CPU_ACCESS_READ,
    CPU_ACCESS_WRITE,
}

#[repr(C)]
pub struct TextureCreationInfo {
    
}

#[repr(C)]
pub struct Texture {
    unk101: u8,
    view: Option<NonNull<u8>>,
    view_synchronizer_vtable: Option<NonNull<u8>>,
}